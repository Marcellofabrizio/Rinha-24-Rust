mod api;
mod db;

use std::{env, sync::Arc};

use db::repository::PgRepository;
use api::app_service::app_service;

type AppState = Arc<PgRepository>;

#[tokio::main]
async fn main() {
    let port = env::var("PORT")
        .ok()
        .and_then(|port| port.parse::<u16>().ok())
        .unwrap_or(9999);

    let database_url = env::var("DATABASE_URL")
        .unwrap_or(String::from("postgres://user:pass@localhost:5432/rinha24"));

    let repo = PgRepository::connect(&database_url)
        .await
        .unwrap();


    let app_state = Arc::new(repo);

    let app = app_service(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
}
