use tracing::{error, info};


mod config;
mod control;
mod routing;
mod service;

use config::CONFIG;

use routing::main_router;

#[tracing::instrument]
pub async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();

    info!("Running server!");
    axum::Server::try_bind(&CONFIG.server_addr)?
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
