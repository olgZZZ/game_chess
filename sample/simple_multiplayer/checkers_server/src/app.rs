use crate::game_store::GameStore;
use std::sync::Mutex;
use actix_cors::Cors;
use actix_web::HttpServer;
use crate::api::{default_route, create_game, join_game};
use actix_web::App;

pub struct AppData {
    pub game_store: Mutex<GameStore>,
}

impl AppData {
    pub fn new() -> Self {
        Self {
            game_store: Mutex::new(GameStore::new()),
        }
    }
}

pub async fn start_app() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_origin()
                    .allow_any_method(),
            )
            .default_service(actix_web::web::route().to(default_route))
            .data(AppData::new())
            .service(create_game)
            .service(join_game)
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}
