use std::sync::Arc;

use sqlx::PgPool;

use std::path;

use axum::{
    extract::{Path, Query, State},
    http::{header, StatusCode},
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};

use crate::db::{
    repository::{self, PgRepository},
    transaction::{self, NewTransaction},
};
use crate::AppState;

pub fn app_service(app_state: Arc<PgRepository>) -> Router {
    Router::new()
        .route("/clientes/:id/transacoes", post(get_extrato))
        .with_state(app_state)
}

pub async fn get_extrato(
    State(transaction): State<AppState>,
    Json(newTransaction): Json<NewTransaction>,
) -> impl IntoResponse {
    println!("{:?}", newTransaction);
    match transaction
        .create_transaction(newTransaction.id_cliente, newTransaction)
        .await
    {
        Ok(id) => {
            return Ok((
                StatusCode::CREATED,
                [(header::LOCATION, format!("/files/{}", id))],
            ));
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
