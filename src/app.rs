use axum::{Router, routing::{get, post}, response::Html};
use crate::routes;

#[derive(Clone)]
pub struct AppState{
    pub pool: sqlx::SqlitePool,   
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/",get(|| async {Html::from("<h1>Welcome to My Page!</h1>")}))
        .route("/commissions",get(routes::get::commissions))
        .route("/commissions/new", post(routes::post::commission_new))
        .route("/commissions/new", get(routes::get::commission_new))
        .route("/commissions/delete/:id", post(routes::post::commissions_delete))
        .with_state(state)
}
