use crate::types::{Game, Player};
use std::collections::HashMap;
use uuid::Uuid;

pub struct GameStore {
    games: HashMap<Uuid, Game>,
}

impl GameStore {
    pub fn new() -> Self {
        Self {
            games: HashMap::new(),
        }
    }

    pub async fn add(&mut self, game: Game) {
        self.games.insert(game.id.clone(), game);
    }

    pub async fn add_player_to_game(&mut self, game_id: &Uuid, player: Player) {
        if let Some(game) = self.games.get_mut(&game_id) {
            game.add_player(player);
        }
    }
}