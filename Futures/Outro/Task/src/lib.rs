// This is the final exercise: build an asynchronous REST API for the ticket
// management system we've grown throughout the course.
//
// The API should let a client create, list, retrieve and patch tickets.
// `Cargo.toml` is yours to edit — pull in whatever crates you like from
// crates.io. We suggest `axum`, `tokio` and `serde`, but you're free to
// choose a different stack. See `task.md` for how to run and test it.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Clone, Serialize, Deserialize)]
pub struct Ticket {
    pub id: u64,
    pub title: String,
    pub description: String,
    pub status: Status,
}

#[derive(Clone, Serialize, Deserialize)]
pub enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Deserialize)]
pub struct CreateTicketRequest {
    title: String,
    description: String,
}

#[derive(Deserialize)]
pub struct PatchTicketRequest {
    title: Option<String>,
    description: Option<String>,
    status: Option<Status>,
}

#[derive(Default)]
pub struct TicketStore {
    tickets: BTreeMap<u64, Ticket>,
    next_id: u64,
}

pub struct AppState {
    store: RwLock<TicketStore>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            store: RwLock::new(TicketStore::default()),
        }
    }
}

pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    let mut store = state.store.write().await;

    let id = store.next_id;
    store.next_id += 1;

    let ticket = Ticket {
        id,
        title: payload.title,
        description: payload.description,
        status: Status::ToDo,
    };
    store.tickets.insert(id, ticket.clone());

    (StatusCode::CREATED, Json(ticket))
}

pub async fn list_tickets(State(state): State<Arc<AppState>>) -> Json<Vec<Ticket>> {
    let store = state.store.read().await;
    Json(store.tickets.values().cloned().collect())
}

pub async fn get_ticket(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, StatusCode> {
    let store = state.store.read().await;
    store
        .tickets
        .get(&id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn patch_ticket(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(payload): Json<PatchTicketRequest>,
) -> Result<Json<Ticket>, StatusCode> {
    let mut store = state.store.write().await;
    let ticket = store.tickets.get_mut(&id).ok_or(StatusCode::NOT_FOUND)?;

    if let Some(title) = payload.title {
        ticket.title = title;
    }
    if let Some(description) = payload.description {
        ticket.description = description;
    }
    if let Some(status) = payload.status {
        ticket.status = status;
    }
    Ok(Json(ticket.clone()))
}
