pub mod chess;

#[cfg(test)]
mod tests
{
  use super::chess::chess_client::ChessClient;
  use crate::generated::chess::{CreateGame, Player};

  #[tokio::test]
  async fn grpc_example()
  {
    let mut grpc_client = ChessClient::connect("http://[::1]:50051").await.unwrap();

    let game_create_request = CreateGame {
      player_id : Some(Player {
        player_id : "6b465716-da28-41c0-a7d7-a8eb9dcb12c5".into(),
        player_name : "Test Player".into(),
      }),
    };
    let result = grpc_client.push_game_create(game_create_request).await;
    println!("{}", result.unwrap().get_ref().game_id);
  }
}
