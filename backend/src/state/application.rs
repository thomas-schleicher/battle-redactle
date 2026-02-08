use std::{collections::HashMap, sync::Arc};

use tokio::sync::RwLock;

use crate::lobby::{Lobby, LobbyId};

#[derive(Default)]
pub struct ApplicationState {
    pub lobbies: RwLock<HashMap<LobbyId, Arc<Lobby>>>,
}
