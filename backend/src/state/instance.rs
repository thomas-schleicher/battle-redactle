use std::collections::HashMap;

use axum::extract::ws::Message;
use tokio::sync::mpsc;
use uuid::Uuid;

pub type InstanceId = String;
pub type PlayerId = Uuid;

pub struct Instance {
    pub players: HashMap<PlayerId, mpsc::UnboundedSender<Message>>,
}

impl Instance {
    pub fn new() -> Self {
        Self {
            players: HashMap::new(),
        }
    }
}
