pub mod app;
pub mod db;
pub mod model;
pub mod routes;
pub mod schemas;
pub mod settings;
pub mod templates;

use app::AppState;
use db::CommissionRepository;
use tracing::info;
use tokio::net::TcpListener;
use settings::Settings;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error>{
    tracing_subscriber::fmt::init();
    let settings = Settings::new()?;

    let pool = db::get_sqlite_pool().await?;
    let commissions = CommissionRepository::new(pool.clone());

    let state = AppState { commissions };
    let router = app::router(state);

    let listener = TcpListener::bind((settings.host, settings.port)).await?;
    info!("{}",listener.local_addr()?);

    axum::serve(listener,router).await?;

    Ok(())
}
