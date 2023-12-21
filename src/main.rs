use std::process;

use anyhow::Context as _;
use axum::Router;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::error;

mod config;
mod error;
mod model;
mod routes;
mod work;

use self::{config::Config, routes::v1_routes};

fn main() {
    let dotenv_res = dotenvy::dotenv();
    tracing_subscriber::fmt::init();
    if let Err(e) = dotenv_res {
        // Log the error but continue
        error!("Failed to load .env: {e}");
    }

    if let Err(e) = async_main() {
        error!("{e}");
        process::exit(1);
    }
}

#[tokio::main]
async fn async_main() -> anyhow::Result<()> {
    let config: Config = envy::from_env().context("reading configuration from environment")?;

    let db_pool = PgPool::connect(&config.database_url).await.context("initializing DB pool")?;

    tokio::spawn({
        let db_pool = db_pool.clone();
        async move {
            work::run_tasks(db_pool).await;
        }
    });

    let router = Router::new().nest("/v1", v1_routes()).with_state(db_pool);

    let listener =
        TcpListener::bind(&config.listen_addr).await.context("settings up TCP listener")?;
    axum::serve(listener, router).await?;

    Ok(())
}
