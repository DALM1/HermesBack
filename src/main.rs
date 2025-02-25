mod websocket;
mod firebase;
mod routes;
mod utils;

use actix_web::{web, App, HttpServer};
use actix_cors::Cors;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    println!("Starting WebSocket server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
