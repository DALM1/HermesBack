mod firebase;
mod routes;

use actix_web::{App, HttpServer};
use actix_cors::Cors;
use env_logger;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    println!("⚡️ HERMES RUN http://127.0.0.1:8080");

    HttpServer::new(|| {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header()
            .supports_credentials();

        App::new()
            .wrap(cors)
            .configure(routes::init)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
