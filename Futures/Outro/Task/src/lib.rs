// This is our last exercise. Let's go down a more unstructured path!
// Try writing an **asynchronous REST API** to expose the functionality
// of the ticket management system we built throughout the course.
// It should expose endpoints to:
//  - Create a ticket
//  - Retrieve ticket details
//  - Patch a ticket
//
// Use Rust's package registry, crates.io, to find the dependencies you need
// (if any) to build this system.

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Serialize, Deserialize)]
pub struct Ticket {
    id: u64,
    title: String,
    description: String,
    status: TicketStatus,
}

impl Ticket {
    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn status(&self) -> &TicketStatus {
        &self.status
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub enum TicketStatus {
    Open,
    InProgress,
    Resolved,
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
    status: Option<TicketStatus>,
}

pub struct AppState {
    tickets: Mutex<Vec<Ticket>>,
    next_id: Mutex<u64>,
}

impl AppState {
    pub fn new(tickets: Mutex<Vec<Ticket>>, next_id: Mutex<u64>) -> Self {
        AppState { tickets, next_id }
    }
}

pub async fn create_ticket(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<CreateTicketRequest>,
) -> impl IntoResponse {
    let mut next_id = state.next_id.lock().await;
    let mut tickets = state.tickets.lock().await;

    let ticket = Ticket {
        id: *next_id,
        title: payload.title,
        description: payload.description,
        status: TicketStatus::Open,
    };

    *next_id += 1;
    tickets.push(ticket.clone());

    (StatusCode::CREATED, Json(ticket))
}

pub async fn get_ticket(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
) -> Result<Json<Ticket>, StatusCode> {
    let tickets = state.tickets.lock().await;

    tickets
        .iter()
        .find(|t| t.id == id)
        .cloned()
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn patch_ticket(
    State(state): State<Arc<AppState>>,
    Path(id): Path<u64>,
    Json(payload): Json<PatchTicketRequest>,
) -> Result<Json<Ticket>, StatusCode> {
    let mut tickets = state.tickets.lock().await;

    if let Some(ticket) = tickets.iter_mut().find(|t| t.id == id) {
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
    } else {
        Err(StatusCode::NOT_FOUND)
    }
}
