use std::sync::Arc;
use axum::Router;
use axum::routing::{get, patch, post};
use tokio::sync::Mutex;
use task_futures_outro::{create_ticket, get_ticket, patch_ticket, AppState};

#[tokio::main]
async fn main() {

    let state = Arc::new(AppState::new(Mutex::new(Vec::new()), Mutex::new(0)));

    let app = Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/{id}", get(get_ticket))
        .route("/tickets/{id}", patch(patch_ticket))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}