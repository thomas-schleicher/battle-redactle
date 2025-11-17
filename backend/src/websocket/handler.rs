use axum::{
    extract::{
        ConnectInfo, Query, State, WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers};
use futures::StreamExt;
use serde::Deserialize;
use std::{net::SocketAddr, sync::Arc};
use tokio::sync::mpsc;

use crate::state::{ApplicationState, Lobby, LobbyId, PlayerId};

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
    mut socket: WebSocket,
    state: Arc<ApplicationState>,
    lobby_id: LobbyId,
    player_uuid: PlayerId,
) {
    // let (tx, mut rx) = mpsc::unbounded_channel::<Message>(); //TODO: figure out how this is different to socket.split()
    let (mut transmit, mut recieve) = socket.split();

    {
        let mut lobbies = state.lobbies.write().await;
        let lobby = lobbies.entry(lobby_id.clone()).or_insert(Lobby::new());
        lobby.players.insert(player_uuid, tx);
    }
}
