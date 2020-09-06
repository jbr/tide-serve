#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    unreachable_pub,
    missing_copy_implementations,
    unused_qualifications
)]

use std::{
    env,
    fmt::{self, Debug, Display},
    path::PathBuf,
};
use structopt::StructOpt;
use tide::{
    http::Url,
    listener::{Listener, ToListener},
    Body, Request, Result, Status,
};
use tide_rustls::TlsListener;

#[derive(Clone)]
struct RootPath(PathBuf);
impl Default for RootPath {
    fn default() -> Self {
        Self(
            env::current_dir()
                .expect("current dir")
                .canonicalize()
                .expect("canonicalize"),
        )
    }
}
impl std::ops::Deref for RootPath {
    type Target = PathBuf;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl std::ops::DerefMut for RootPath {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for RootPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.0, f)
    }
}

impl Display for RootPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.to_str().unwrap())
    }
}

impl From<&std::ffi::OsStr> for RootPath {
    fn from(s: &std::ffi::OsStr) -> Self {
        Self(PathBuf::from(s).canonicalize().expect("canonicalize"))
    }
}
#[derive(StructOpt, Debug)]
#[structopt(
    name = "tide-serve",
    about = "a simple static http server built with tide"
)]
struct Options {
    #[structopt(short = "o", long, env)]
    host: Option<String>,

    #[structopt(short, long, env)]
    port: Option<u16>,

    #[structopt(parse(from_os_str), default_value)]
    root: RootPath,

    #[structopt(short, long, env = "TIDE_BIND")]
    bind: Option<Url>,

    #[structopt(short, long, env, parse(from_os_str))]
    cert_path: Option<PathBuf>,

    #[structopt(short, long, env, parse(from_os_str))]
    key_path: Option<PathBuf>,
}

struct DynListener<T> {
    l: Box<dyn Listener<T>>,
}

impl<T> Display for DynListener<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Display::fmt(&self.l, f)
    }
}

impl<T> Debug for DynListener<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        Debug::fmt(&self.l, f)
    }
}

impl<T: Clone + Send + Sync + 'static> DynListener<T> {
    fn new<TL, L>(tl: TL) -> Self
    where
        TL: ToListener<T, Listener = L>,
        L: Listener<T>,
    {
        Self {
            l: Box::new(tl.to_listener().unwrap()),
        }
    }
}

#[tide::utils::async_trait]
impl<T: Clone + Send + Sync + 'static> Listener<T> for DynListener<T> {
    async fn listen(&mut self, app: tide::Server<T>) -> std::io::Result<()> {
        self.l.listen(app).await
    }
}

impl<T: Clone + Send + Sync + 'static> ToListener<T> for DynListener<T> {
    type Listener = DynListener<T>;
    fn to_listener(self) -> std::io::Result<Self::Listener> {
        Ok(self)
    }
}

const LOCALHOST: &str = "127.0.0.1";
impl Options {
    fn listener<T: Clone + Send + Sync + 'static>(&self) -> DynListener<T> {
        match self {
            Options {
                cert_path: Some(cert),
                key_path: Some(key),
                host,
                port,
                ..
            } => DynListener::new(
                TlsListener::build()
                    .addrs((
                        host.as_deref().unwrap_or(LOCALHOST),
                        port.or_else(portpicker::pick_unused_port).unwrap(),
                    ))
                    .key(key)
                    .cert(cert),
            ),

            Options {
                bind: Some(bind), ..
            } => DynListener::new(bind.clone()),

            Options {
                host,
                port: Some(port),
                ..
            } => DynListener::new((host.as_deref().unwrap_or(LOCALHOST), *port)),

            _ => DynListener::new((LOCALHOST, portpicker::pick_unused_port().unwrap())),
        }
    }
}

#[async_std::main]
async fn main() -> Result<()> {
    let options = Options::from_args();
    let path = options.root.clone();
    let mut app = tide::with_state(path.clone());

    app.with(driftwood::DevLogger);

    app.at("/")
        .get(|req: Request<RootPath>| async move {
            Ok(Body::from_file(req.state().join("index.html"))
                .await
                .status(404)?)
        })
        .serve_dir(&*path)?;

    let listener = options.listener();
    println!("serving {} on {}", options.root, listener);
    app.listen(listener).await?;
    Ok(())
}
