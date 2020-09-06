use crate::{DynListener, RootPath};
use std::{fmt::Debug, path::PathBuf};
use structopt::StructOpt;
use tide_rustls::TlsListener;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "tide-serve",
    about = "a simple static http server built with tide",
    setting = structopt::clap::AppSettings::DeriveDisplayOrder
)]
pub struct CliOptions {
    /// Filesystem path to serve
    ///
    /// Defaults to the current working directory
    #[structopt(parse(from_os_str), default_value)]
    root: RootPath,

    /// Local host or ip to listen on
    #[structopt(short = "o", long, env, default_value = "localhost")]
    host: String,

    /// Local port to listen on
    #[structopt(short, long, env, default_value = "8080")]
    port: u16,

    /// Local listener spec to bind
    ///
    /// Examples:
    ///     `--bind localhost:8080`
    ///     `--bind http://localhost:8080`
    ///     `--bind [::1]:1213`
    ///
    /// On unix-like systems only:
    ///     `--bind http+unix:///var/run/some.socket`
    ///     `--bind http+unix://./tmp/socket`
    ///
    /// --bind will override --host and --port.
    #[structopt(short, long, env = "TIDE_BIND")]
    bind: Option<String>,

    /// Path to a tls certificate for tide_rustls
    ///
    /// This will be ignored unless key_path is also
    /// provided. providing both key_path and cert_path enables tls.
    ///
    /// Example: `--cert ./cert.pem --key ./key.pem`
    /// For development, try using mkcert
    #[structopt(short, long, env, parse(from_os_str))]
    cert_path: Option<PathBuf>,

    /// The path to a tls key file for tide_rustls
    ///
    /// This will be ignored unless cert_path is also
    /// provided. providing both key_path and cert_path enables tls.
    ///
    /// Example: `--cert ./cert.pem --key ./key.pem`
    /// For development, try using mkcert
    #[structopt(short, long, env, parse(from_os_str))]
    key_path: Option<PathBuf>,

    /// Host to forward (reverse proxy) not-found requests to
    ///
    /// This forwards any request that would otherwise be a 404 Not
    /// Found to the specified listener spec.
    ///
    /// Examples:
    ///    `--forward localhost:8081`
    ///    `--forward http://localhost:8081`
    ///    `--forward https://localhost:8081`
    ///
    /// Note: http+unix:// schemes are not yet supported
    #[structopt(short, long, env = "FORWARD")]
    forward: Option<String>,
}

impl CliOptions {
    pub fn root(&self) -> &RootPath {
        &self.root
    }

    pub fn forward(&self) -> Option<&str> {
        self.forward.as_deref()
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
                    .addrs((&host[..], *port))
                    .key(key)
                    .cert(cert),
            ),

            CliOptions {
                bind: Some(bind), ..
            } => DynListener::new(bind.clone()),

            CliOptions { host, port, .. } => DynListener::new((&host[..], *port)),
        }
    }
}
