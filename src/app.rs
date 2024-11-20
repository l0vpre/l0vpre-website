use crate::routes;
use crate::db::CommissionRepository;
use axum::{handler::HandlerWithoutStateExt, routing::{get, post}, Router};
use tower_http::services::ServeDir;

#[derive(Clone)]
pub struct AppState {
    pub commissions: CommissionRepository,
}

pub fn router(state: AppState) -> Router {
    let fallback = ServeDir::new("static")
        .fallback(routes::get::not_found.into_service());

    Router::new()
        .route("/",get(routes::get::index))
        .route("/commissions",get(routes::get::commissions))
        .route("/commissions/new", post(routes::post::commission_new))
        .route("/commissions/new", get(routes::get::commission_new))
        .route("/commissions/delete/:id", post(routes::post::commissions_delete))
        .route("/portfolio", get(routes::get::portfolio))
        .route("/oc", get(routes::get::oc))
        .with_state(state)
        .fallback_service(fallback)
}
