use axum::extract::{State, Form, Path};
use axum::response::{Redirect,IntoResponse};
use tracing::error;
use crate::app::AppState;
use crate::db::{get_commissions, insert_commission, delete_commission_by_id, CommissionInsertData};
use crate::schemas::NewCommissionForm;
use crate::templates::{CommissionTemplate, CommissionNewTemplate};


pub mod get {
    use super::*;

    pub async fn commissions(State(state): State<AppState>) -> CommissionTemplate {
        let commissions = get_commissions(&state.pool).await.unwrap_or_default();
        CommissionTemplate { commissions }
    }

    pub async fn commission_new() -> CommissionNewTemplate {
        CommissionNewTemplate{}
    }
}

pub mod post {

    use super::*;

    pub async fn commission_new(State(state): State<AppState>, Form(form) : Form<NewCommissionForm>) -> impl IntoResponse {
        let NewCommissionForm{ title, contact, description } = form;
        let result =insert_commission(
            &state.pool,
            CommissionInsertData { title, contact, description, status: "something".to_owned() }
        ).await;

        if let Err(e) = result{
            error!("Error inserting new commission: {e}");
        }

        Redirect::to("/commissions")
    }

    pub async fn commissions_delete(State(state): State<AppState>, Path(id): Path<i64>) -> impl IntoResponse {
        let result = delete_commission_by_id(&state.pool, id).await;

        if let Err(e) = result{
            error!("Error deleting commission: {e}");
        }

        Redirect::to("/commissions")

    }
}
