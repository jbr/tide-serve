use std::{env, path::PathBuf};
use tide::{Body, Request, Result, Status};

#[async_std::main]
async fn main() -> Result<()> {
    let path = env::args().nth(1).unwrap_or(String::from("."));
    let path = PathBuf::from(path).canonicalize().unwrap();
    let mut app = tide::with_state(path.clone());

    tide::log::start();
    tide::log::info!("serving {:?}", path);

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
