use axum::{
    extract::{State, Query},
    http::StatusCode,
    routing::{get, patch, post},
    Json, Router,
};
use outro_08::data::{TicketDraft, TicketId, TicketPatch};
use outro_08::store::TicketStore;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, RwLock};

#[tokio::main]
async fn main() {
    let store: TicketStore = TicketStore::new();
    let shared_store = Arc::new(RwLock::new(store));

    // build our application with a single route
    let app = Router::new()
        .route("/", get(|| async { "Hello, Welcome to TicketStore!" }))
        .route("/create", post(create_ticket))
        .route("/patch", patch(patch_ticket))
        .route("/get", get(get_ticket))
        .with_state(shared_store.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn create_ticket(
    State(state): State<Arc<RwLock<TicketStore>>>,
    Json(payload): Json<TicketDraft>,
) -> (StatusCode, Json<Message>) {
    // Try to acquire the write lock, return an error if it fails
    let id = match state.write() {
        Ok(mut store) => store.add_ticket(payload),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: "Failed to acquire lock on ticket store".to_string(),
                }),
            );
        }
    };

    let message = Message {
        message: format!("Ticket created with ID: {}", id.0),
    };
    (StatusCode::CREATED, Json(message))
}

async fn patch_ticket(
    State(state): State<Arc<RwLock<TicketStore>>>,
    Query(params): Query<TicketParams>,
    Json(payload): Json<TicketPatch>,
) -> (StatusCode, Json<Message>) {
    let ticket = match state.read() {
        Ok(store) => store.get(params.id),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: "Failed to acquire lock on ticket store".to_string(),
                })
            );
        }
    };
    if let Some(t) = ticket {
        let mut ticket = t.write().unwrap();
        ticket.title = payload.title;
        ticket.description = payload.description;
        ticket.status = payload.status;
        return (
            StatusCode::OK,
            Json(Message {
                message: format!("Ticket {} updated", params.id),
            })
        );
    } else {
        return (
            StatusCode::NOT_FOUND,
            Json(Message {
                message: "Ticket not found".to_string(),
            }),
        );
    }
}

async fn get_ticket(
    State(state): State<Arc<RwLock<TicketStore>>>,
    Query(params): Query<TicketParams>,
) -> (StatusCode, Json<Message>) {
    let ticket = match state.read() {
        Ok(store) => store.get(params.id),
        Err(_) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(Message {
                    message: "Failed to acquire lock on ticket store".to_string(),
                }),
            );
        }
    };
    match ticket {
        Some(t) => (
            StatusCode::OK,
            Json(Message {
                message: format!("{}", t.read().unwrap().to_string()),
            }),
        ),
        None => (
            StatusCode::NOT_FOUND,
            Json(Message {
                message: "Ticket not found".to_string(),
            }),
        ),
    }
}

#[derive(Debug, Serialize)]
struct Message {
    message: String,
}

#[derive(Debug, Deserialize)]
struct TicketParams {
    id: TicketId,
}