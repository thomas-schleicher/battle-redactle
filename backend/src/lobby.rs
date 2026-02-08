use std::collections::HashMap;

use axum::extract::ws::Message;
use tokio::sync::{RwLock, mpsc::UnboundedSender};
use uuid::Uuid;

use crate::websocket::{ClientMessage, ServerMessage};

pub type LobbyId = String;
pub type PlayerId = Uuid;

pub struct Lobby {
    players: RwLock<HashMap<PlayerId, UnboundedSender<Message>>>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            players: RwLock::new(HashMap::new()),
        }
    }

    pub async fn message(&self, player_id: &PlayerId, msg: ServerMessage) {
        if let Some(player_tx) = self.players.read().await.get(player_id) {
            let json = match serde_json::to_string(&msg) {
                Ok(j) => j,
                Err(e) => {
                    eprintln!("Failed to serialize ServerMessage: {:?}", e);
                    return;
                }
            };

            let _ = player_tx.send(Message::Text(json.into()));
        } else {
            eprintln!("No such player {:?}", player_id);
        }
    }

    pub async fn broadcast(&self, _msg: ServerMessage) {
        //TODO: send message to all clients in lobby
    }

    pub async fn handle_client_message(&self, _msg: Message) {
        //TODO: cast message to client message ither here or in handler
    }

    pub async fn add_player(&self, player_id: PlayerId, tx: UnboundedSender<Message>) {
        let mut players = self.players.write().await;
        players.insert(player_id, tx);
    }

    pub async fn remove_player(&self, player_id: &PlayerId) {
        let mut players = self.players.write().await;
        players.remove(player_id);
    }
}
