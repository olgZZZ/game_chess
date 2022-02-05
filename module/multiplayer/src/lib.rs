pub mod generated;

use time::OffsetDateTime;
use crate::generated::chess::Player;

///
/// Message.
///

#[allow(dead_code)]
pub struct MultiplayerMessage {
    player_id: String,
    text: String,
    timestamp: OffsetDateTime,
}

impl MultiplayerMessage {

}

///
/// Player.
///

#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiplayerPlayer {
    id: String,
    name: String,
}

impl MultiplayerPlayer {

}

impl From<&Player> for MultiplayerPlayer {
    fn from(p: &Player) -> Self {
        Self {
            id: p.player_id.clone(),
            name: p.player_name.clone(),
        }
    }
}

///
/// Move.
///

pub struct MultiplayerMove {
    /// Player id.
    pub player_id: String,
    /// Game id.
    pub game_id: String,
}

impl MultiplayerMove {

}

///
/// Multiplayer game.
///

#[allow(dead_code)]
#[derive(Debug)]
pub struct MultiplayerGame {
    id: String,
    players: Vec<MultiplayerPlayer>,
}

impl MultiplayerGame {
    pub fn new(id: &str, player: MultiplayerPlayer) -> Self {
        Self {
            id: id.into(),
            players: vec![player],
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::generated::chess::chess_client::ChessClient;
    use crate::generated::chess::{CreateGame, Player};
    use tonic::Request;

    #[tokio::test]
    async fn test_multiplayer() {
        let mut chess_client = ChessClient::connect("http://127.0.0.1:1313").await.unwrap();

        let res = chess_client.pull_games_list(Request::new(())).await.unwrap();
        println!("games on the server: {:?}", res.get_ref().game_ids);

        let res = chess_client.push_game_create(Request::new(CreateGame {
            player: Some(Player {
                player_id: "1".into(),
                player_name: "pasha".into(),
            })
        })).await.unwrap();

        println!("created game id: {}", res.get_ref().game_id);

        let res = chess_client.pull_games_list(Request::new(())).await.unwrap();
        println!("games on the server: {:?}", res.get_ref().game_ids);
    }
}
