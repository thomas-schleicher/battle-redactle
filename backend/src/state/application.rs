use std::collections::HashMap;

use axum::extract::ws::WebSocket;
use tokio::sync::RwLock;

use crate::state::instance::{Instance, InstanceId, PlayerId};

#[derive(Default)]
pub struct ApplicationState {
    pub instances: RwLock<HashMap<InstanceId, Instance>>,
}

impl ApplicationState {
    pub async fn join_player(
        &self,
        instance_id: InstanceId,
        player_id: PlayerId,
        socket: WebSocket,
    ) {
    }
}
