mod state;
mod websocket;

use std::{net::SocketAddr, sync::Arc};

use axum::Router;

use crate::{state::ApplicationState, websocket::ws_routes};

#[tokio::main]
async fn main() {
    let state = Arc::new(ApplicationState::default());

    let app = Router::new().merge(ws_routes(state.clone()));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    axum::serve(
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
    .await
    .unwrap();
}
