use axum::{Router, routing::{get, post}};
use crate::routes;
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState{
    pub pool: sqlx::SqlitePool,   
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/",get(routes::get::index))
        .route("/commissions",get(routes::get::commissions))
        .route("/commissions/new", post(routes::post::commission_new))
        .route("/commissions/new", get(routes::get::commission_new))
        .route("/commissions/delete/:id", post(routes::post::commissions_delete))
        .with_state(state)
        .fallback_service(ServeDir::new("static"))
}
