use crate::{DynListener, RootPath};
use std::{fmt::Debug, path::PathBuf};
use structopt::StructOpt;
use tide_rustls::TlsListener;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tide-serve",
    about = "a simple static http server built with tide"
)]
pub struct CliOptions {
    #[structopt(short = "o", long, env)]
    host: Option<String>,

    #[structopt(short, long, env, default_value = "8080")]
    port: u16,

    #[structopt(parse(from_os_str), default_value)]
    root: RootPath,

    #[structopt(short, long, env = "TIDE_BIND")]
    bind: Option<String>,

    #[structopt(short, long, env, parse(from_os_str))]
    cert_path: Option<PathBuf>,

    #[structopt(short, long, env, parse(from_os_str))]
    key_path: Option<PathBuf>,
}

const LOCALHOST: &str = "127.0.0.1";
impl CliOptions {
    pub fn root(&self) -> &RootPath {
        &self.root
    }

    pub fn listener<T: Clone + Send + Sync + 'static>(&self) -> DynListener<T> {
        match self {
            CliOptions {
                cert_path: Some(cert),
                key_path: Some(key),
                host,
                port,
                ..
            } => DynListener::new(
                TlsListener::build()
                    .addrs((host.as_deref().unwrap_or(LOCALHOST), *port))
                    .key(key)
                    .cert(cert),
            ),

            CliOptions {
                bind: Some(bind), ..
            } => DynListener::new(bind.clone()),

            CliOptions { host, port, .. } => {
                DynListener::new((host.as_deref().unwrap_or(LOCALHOST), *port))
            }
        }
    }
}
