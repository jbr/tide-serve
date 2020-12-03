#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    missing_copy_implementations,
    unused_qualifications
)]

mod cli_options;
mod dyn_listener;
mod forward_middleware;
mod root_path;

use cli_options::CliOptions;
use dyn_listener::DynListener;
use forward_middleware::ForwardMiddleware;
use root_path::RootPath;

use structopt::StructOpt;
use tide::{listener::Listener, Body, Request, Result, Status};

#[async_std::main]
async fn main() -> Result<()> {
    let options = CliOptions::from_args();
    let path = options.root().clone();
    let mut app = tide::with_state(path.clone());

    app.with(driftwood::DevLogger);

    if let Some(forward) = options.forward() {
        app.with(ForwardMiddleware::new(forward));
    }

    app.at("/")
        .get(|req: Request<RootPath>| async move {
            Ok(Body::from_file(req.state().join("index.html"))
                .await
                .status(404)?)
        })
        .serve_dir(&*path)?;

    let mut listener = options.listener();
    listener.bind(app).await?;
    println!("serving {} on {}", options.root(), listener);
    listener.accept().await?;
    Ok(())
}
