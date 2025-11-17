use std::collections::HashMap;

use axum::extract::ws::{Message, Utf8Bytes};
use serde::Serialize;
use tokio::sync::mpsc::UnboundedSender;
use uuid::Uuid;

use crate::websocket::{ClientMessage, ServerMessage};

pub type LobbyId = String;
pub type PlayerId = Uuid;

pub struct Lobby {
    players: HashMap<PlayerId, UnboundedSender<Message>>,
}

impl Lobby {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }

    pub async fn message(&self, player_id: &PlayerId, msg: ServerMessage) {
        if let Some(player_tx) = self.players.get(player_id) {
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

    pub async fn broadcast(&self, msg: ServerMessage) {}

    pub async fn handle_client_message(&self, msg: ClientMessage) {}

    pub fn add_player(
        &mut self,
        player_id: PlayerId,
        tx: UnboundedSender<Message>,
    ) -> Option<UnboundedSender<Message>> {
        self.players.insert(player_id, tx)
    }

    pub fn remove_player(
        &mut self,
        player_id: &PlayerId,
    ) -> Option<(PlayerId, UnboundedSender<Message>)> {
        self.players.remove_entry(player_id)
    }
}
