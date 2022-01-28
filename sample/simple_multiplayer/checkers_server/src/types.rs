use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Player {
    pub id: Uuid,
    pub name: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Game {
    pub id: Uuid,
    pub players: Vec<Player>,
}

impl Game {
    pub fn new_with_player(player: Player) -> Self {
        Self {
            id: Uuid::new_v4(),
            players: vec![player],
        }
    }

    pub fn add_player(&mut self, player: Player) {
        self.players.push(player);
    }
}
