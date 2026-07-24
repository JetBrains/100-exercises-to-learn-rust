use axum::routing::{get, post};
use axum::Router;
use std::sync::Arc;
use task_futures_outro::{create_ticket, get_ticket, list_tickets, patch_ticket, AppState};

#[tokio::main]
async fn main() {
    let state = Arc::new(AppState::new());

    let app = Router::new()
        .route("/", get(|| async { "Ticket management API. Try GET /tickets" }))
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/{id}", get(get_ticket).patch(patch_ticket))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");

    axum::serve(listener, app).await.unwrap();
}
