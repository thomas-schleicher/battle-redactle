use axum::{
    extract::{
        ConnectInfo, Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers};
use futures::{SinkExt, StreamExt};
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;

use crate::lobby::{Lobby, LobbyId, PlayerId};
use crate::state::ApplicationState;

#[derive(Deserialize)]
pub struct WsConnectionQuery {
    lobby_id: LobbyId,
    player_uuid: PlayerId,
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<headers::UserAgent>>,
    Query(query): Query<WsConnectionQuery>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
    State(state): State<Arc<ApplicationState>>,
) -> impl IntoResponse {
    //TODO: figure out how to best do logging and also if the conect info is even needed
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        String::from("Unknown browser")
    };

    println!("`{user_agent}` at {addr} connected.");

    ws.on_upgrade(move |socket| handle_socket(socket, state, query.lobby_id, query.player_uuid))
}

async fn handle_socket(
    socket: WebSocket,
    state: Arc<ApplicationState>,
    lobby_id: LobbyId,
    player_uuid: PlayerId,
) {
    let (mut transmit, mut receive) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<Message>();

    let lobby = {
        let mut lobbies = state.lobbies.write().await;
        let lobby = lobbies
            .entry(lobby_id.clone())
            .or_insert_with(|| Arc::new(Lobby::new()));
        lobby.add_player(player_uuid, tx).await;
        // We clone the Arc<Lobby> to move it into the tasks
        Arc::clone(lobby)
    };

    let mut send_task = tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            if transmit.send(message).await.is_err() {
                break;
            }
        }
    });

    let lobby_for_recv = Arc::clone(&lobby);
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receive.next().await {
            // This now works because lobby_for_recv is an Arc (owned)
            lobby_for_recv.handle_client_message(msg).await;
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    };

    let mut lobbies = state.lobbies.write().await;
    if let Some(lobby) = lobbies.get_mut(&lobby_id) {
        lobby.remove_player(&player_uuid).await;
    }
}
