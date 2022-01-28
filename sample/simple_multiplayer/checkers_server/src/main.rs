#[actix_web::main]
async fn main() -> std::io::Result<()> {
    checkers_server::app::start_app().await
}