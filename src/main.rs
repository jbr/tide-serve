#![forbid(unsafe_code, future_incompatible)]
#![deny(
    missing_debug_implementations,
    nonstandard_style,
    unreachable_pub,
    missing_copy_implementations,
    unused_qualifications
)]

use std::path::PathBuf;
use tide::{Body, Request, Result, Status};

#[async_std::main]
async fn main() -> Result<()> {
    let path = std::env::args().nth(1).unwrap_or_else(|| String::from("."));
    let path = PathBuf::from(path).canonicalize().unwrap();
    let mut app = tide::with_state(path.clone());

    println!("serving {:?} at http://localhost:8000/", path);

    app.with(driftwood::DevLogger);

    app.at("/")
        .get(|req: Request<PathBuf>| async move {
            Ok(Body::from_file(req.state().join("index.html"))
                .await
                .status(404)?)
        })
        .serve_dir(path)?;

    app.listen("tcp://localhost:8000").await?;

    Ok(())
}
