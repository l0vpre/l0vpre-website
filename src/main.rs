mod db;
mod templates;
mod routes;
mod app;
mod schemas;
mod settings;

use tracing::info;
use tokio::net::TcpListener;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    tracing_subscriber::fmt::init();
    let settings = Settings::new()?;
    let app_state = app::AppState{ pool: db::get_sqlite_pool().await?};
    let router = app::router(app_state);
    let listener = TcpListener::bind((settings.host, settings.port)).await?;
    info!("{}",listener.local_addr()?);
    axum::serve(listener,router).await?;
    Ok(())
}
