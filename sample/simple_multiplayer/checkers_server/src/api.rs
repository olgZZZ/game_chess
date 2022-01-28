use actix_web::{HttpRequest, Responder, HttpResponse};
use actix_web::http::StatusCode;
use actix_web::{post, web};
use crate::app::AppData;
use crate::types::{Game, Player};
use uuid::Uuid;
use std::str::FromStr;
use serde::Deserialize;

pub async fn default_route(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok().status(StatusCode::OK).body(())
}

#[post("/create")]
pub async fn create_game(player: web::Json<Player>, app: web::Data<AppData>) -> impl Responder {
    let game = Game::new_with_player(player.clone());
    app.game_store.lock().unwrap().add(game.clone()).await;

    HttpResponse::Ok().status(StatusCode::CREATED).body(serde_json::to_string(&game).unwrap())
}

#[derive(Deserialize)]
pub struct JoinGamePathParameters {
    pub game_id: String,
}

#[post("/join/{game_id}")]
pub async fn join_game(params: web::Path<JoinGamePathParameters>, player: web::Json<Player>, app: web::Data<AppData>) -> impl Responder {
    app.game_store.lock().unwrap().add_player_to_game(&Uuid::from_str(&params.game_id).unwrap(), player.clone()).await;

    HttpResponse::Ok().body(())
}
