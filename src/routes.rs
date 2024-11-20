use axum::{extract::{Form, Path, State}, response::{IntoResponse, Redirect}};
use tracing::error;
use crate::{
    app::AppState,
    db::CommissionInsertData,
    schemas::NewCommissionForm,
    templates::{CommissionNewTemplate, CommissionTemplate, IndexTemplate, OCTemplate, PortfolioTemplate, NotFoundTemplate}
};


pub mod get {


    use super::*;

    pub async fn index() -> IndexTemplate {
        IndexTemplate{}
    }

    pub async fn commissions(State(state): State<AppState>) -> CommissionTemplate {
        let commissions = state.commissions.get_all().await.unwrap_or_default();
        CommissionTemplate { commissions }
    }

    pub async fn commission_new() -> CommissionNewTemplate {
        CommissionNewTemplate{}
    }

    pub async fn portfolio() -> PortfolioTemplate{
        PortfolioTemplate{}
    }

    pub async fn oc() -> OCTemplate{
        OCTemplate{}
    }

    pub async fn not_found() -> NotFoundTemplate {
        NotFoundTemplate {}
    }
}

pub mod post {

    use super::*;

    pub async fn commission_new(State(state): State<AppState>, Form(form) : Form<NewCommissionForm>) -> impl IntoResponse {
        let NewCommissionForm{ title, contact, description } = form;
        let result = state.commissions.insert(
            CommissionInsertData { title, contact, description, status: "something".to_owned() }
        ).await;

        if let Err(e) = result{
            error!("Error inserting new commission: {e}");
        }

        Redirect::to("/commissions")
    }

    pub async fn commissions_delete(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
        let result = state.commissions.delete_by_id(id).await;

        if let Err(e) = result{
            error!("Error deleting commission: {e}");
        }

        Redirect::to("/commissions")

    }
}
