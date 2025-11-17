use serde::{Deserialize, Serialize};

use crate::state::{LobbyId, PlayerId};

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ClientMessage {
    Join {
        instance_id: LobbyId,
        player_id: PlayerId,
        player_name: String,
    },
    Leave,
    Guess {
        guess: String,
    },
    UsePowerUp {
        // powerup: todo!("Not implemented yet!"),
    },
    // GameStateRequest,
}

#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ServerMessage {
    Joined {
        player_name: String,
    },
    Left {
        player_name: String,
    },
    GuessResult {
        guess: String,
        found: bool,
        positions: Vec<u32>,
    },
    GameState {
        //Todo: create game state repesentation
    },
}
