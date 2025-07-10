#[cfg(test)]
mod tests {
    use axum::routing::{get, patch, post};
    use axum::{
        body::Body,
        http::{Method, Request, StatusCode},
        Router,
    };
    use std::sync::Arc;
    use task_futures_outro::*;
    use tokio::sync::Mutex;
    use tower::ServiceExt;

    async fn setup_app() -> Router {
        let state = Arc::new(AppState::new(Mutex::new(Vec::new()), Mutex::new(0)));

        Router::new()
            .route("/tickets", post(create_ticket))
            .route("/tickets/{id}", get(get_ticket))
            .route("/tickets/{id}", patch(patch_ticket))
            .with_state(state)
    }

    #[tokio::test]
    async fn test_create_ticket() {
        let app = setup_app().await;

        let request = Request::builder()
            .method(Method::POST)
            .uri("/tickets")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "Test Ticket",
                    "description": "Test Description"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::CREATED);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let ticket: Ticket = serde_json::from_slice(&body).unwrap();

        assert_eq!(ticket.id(), 0);
        assert_eq!(ticket.title(), "Test Ticket");
        assert_eq!(ticket.description(), "Test Description");
        matches!(ticket.status(), TicketStatus::Open);
    }

    #[tokio::test]
    async fn test_get_ticket() {
        let app = setup_app().await;

        // First create a ticket
        let create_request = Request::builder()
            .method(Method::POST)
            .uri("/tickets")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "Test Ticket",
                    "description": "Test Description"
                })
                .to_string(),
            ))
            .unwrap();

        let _ = app.clone().oneshot(create_request).await.unwrap();

        // Then try to get it
        let get_request = Request::builder()
            .method(Method::GET)
            .uri("/tickets/0")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(get_request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let ticket: Ticket = serde_json::from_slice(&body).unwrap();

        assert_eq!(ticket.id(), 0);
        assert_eq!(ticket.title(), "Test Ticket");
        assert_eq!(ticket.description(), "Test Description");
    }

    #[tokio::test]
    async fn test_get_nonexistent_ticket() {
        let app = setup_app().await;

        let request = Request::builder()
            .method(Method::GET)
            .uri("/tickets/999")
            .body(Body::empty())
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_patch_ticket() {
        let app = setup_app().await;

        // First create a ticket
        let create_request = Request::builder()
            .method(Method::POST)
            .uri("/tickets")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "Test Ticket",
                    "description": "Test Description"
                })
                .to_string(),
            ))
            .unwrap();

        let _ = app.clone().oneshot(create_request).await.unwrap();

        // Then patch it
        let patch_request = Request::builder()
            .method(Method::PATCH)
            .uri("/tickets/0")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "Updated Title",
                    "status": "InProgress"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(patch_request).await.unwrap();

        assert_eq!(response.status(), StatusCode::OK);

        let body = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let ticket: Ticket = serde_json::from_slice(&body).unwrap();

        assert_eq!(ticket.id(), 0);
        assert_eq!(ticket.title(), "Updated Title");
        assert_eq!(ticket.description(), "Test Description"); // Should remain unchanged
        matches!(ticket.status(), TicketStatus::InProgress);
    }

    #[tokio::test]
    async fn test_patch_nonexistent_ticket() {
        let app = setup_app().await;

        let request = Request::builder()
            .method(Method::PATCH)
            .uri("/tickets/999")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "Updated Title"
                })
                .to_string(),
            ))
            .unwrap();

        let response = app.oneshot(request).await.unwrap();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[tokio::test]
    async fn test_create_multiple_tickets() {
        let app = setup_app().await;

        // Create first ticket
        let request1 = Request::builder()
            .method(Method::POST)
            .uri("/tickets")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "First Ticket",
                    "description": "First Description"
                })
                .to_string(),
            ))
            .unwrap();

        let response1 = app.clone().oneshot(request1).await.unwrap();
        assert_eq!(response1.status(), StatusCode::CREATED);

        // Create second ticket
        let request2 = Request::builder()
            .method(Method::POST)
            .uri("/tickets")
            .header("Content-Type", "application/json")
            .body(Body::from(
                serde_json::json!({
                    "title": "Second Ticket",
                    "description": "Second Description"
                })
                .to_string(),
            ))
            .unwrap();

        let response2 = app.clone().oneshot(request2).await.unwrap();
        assert_eq!(response2.status(), StatusCode::CREATED);

        let body2 = axum::body::to_bytes(response2.into_body(), usize::MAX)
            .await
            .unwrap();
        let ticket2: Ticket = serde_json::from_slice(&body2).unwrap();

        assert_eq!(ticket2.id(), 1); // Should have incremented ID
    }
}
