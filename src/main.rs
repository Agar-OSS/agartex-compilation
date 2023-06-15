use tokio::fs;
use tracing::{error, info};

mod constants;
mod control;
mod domain;
mod extract;
mod repository;
mod routing;
mod service;
mod utils;

use routing::main_router;

use constants::SERVER_URL;

use crate::constants::TMP_FILE_DIR;

#[tracing::instrument]
pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    fs::create_dir_all(TMP_FILE_DIR.as_path()).await.unwrap();

    info!("Running server!");
    axum::Server::try_bind(&SERVER_URL)?
        .serve(main_router().into_make_service())
        .await
        .map_err(anyhow::Error::from)
}

#[tokio::main]
#[tracing::instrument]
async fn main() {
    if let Err(err) = run().await {
        error!(%err);
    }
}
