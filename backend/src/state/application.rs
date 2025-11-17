use std::collections::HashMap;

use tokio::sync::RwLock;

use crate::lobby::{Lobby, LobbyId};

#[derive(Default)]
pub struct ApplicationState {
    pub lobbies: RwLock<HashMap<LobbyId, Lobby>>,
}
