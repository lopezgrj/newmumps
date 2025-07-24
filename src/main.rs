mod domain {
    pub mod patient;
    pub mod user;
    pub mod institution;
    pub mod facility;
    pub mod provider;
}


use axum::{Router, routing::{get, post, put, delete}, extract::{Path, State}, Json};
use sqlx::sqlite::SqlitePool;
mod dto {
    pub mod patient_dto;
}
use dto::patient_dto::{PatientInput, PatientOut};
use std::net::SocketAddr;
use std::sync::Arc;


#[tokio::main]
async fn main() {
    // Set up SQLite connection pool
    let db_url = "sqlite://local.db";
    let pool = SqlitePool::connect(db_url).await.expect("Failed to connect to SQLite");
    let pool = Arc::new(pool);



    // Health check
    async fn health() -> &'static str {
        "OK"
    }

    // ...existing code...

mod handlers {
    pub mod patient_handler;
}
use handlers::patient_handler::{list_patients, get_patient, create_patient, update_patient, delete_patient};

    let app = Router::new()
        .route("/", get(|| async { "Axum with SQLite is running!" }))
        .route("/health", get(health))
        .route("/patients", get(list_patients).post(create_patient))
        .route("/patients/:id", get(get_patient).put(update_patient).delete(delete_patient))
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    // Use axum's server startup (Axum 0.6)
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
