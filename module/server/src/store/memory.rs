//!
//! Implements in-memory storage.
//!

#[allow(unused_imports)]
use tonic::async_trait;
use multiplayer::MultiplayerGame as Game;

use crate::store::GameStore;

///
/// Storage structure.
///

#[derive(Debug)]
pub struct MemoryStore
{
  games : Vec<Game>,
}

impl MemoryStore
{
  ///
  /// Storage constructor.
  ///
  pub fn new() -> Self { Self { games : Vec::new() } }
}

impl GameStore for MemoryStore
{
  ///
  /// Add game to storage.
  ///
  fn add_game(&mut self, game : Game) { self.games.push(game); }

  ///
  /// Get game from storage by string ( slice ) id.
  ///
  fn get_game(&self, _game_id : &str) -> &Game { todo!() }

  ///
  /// Get all stored games.
  ///
  fn get_games(&self) -> &Vec<Game> { &self.games }

  ///
  /// Update game in storage using string id and new instance of Game.
  ///
  fn update_game(&mut self, _game_id : &str, _new_game : Game) { todo!() }
}
