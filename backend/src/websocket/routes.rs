use crate::state::ApplicationState;

use super::handler::ws_handler;
use std::sync::Arc;

use axum::{Router, routing::any};

pub fn ws_routes(state: Arc<ApplicationState>) -> Router {
    Router::new()
        .route("/ws", any(ws_handler))
        .with_state(state)
}
