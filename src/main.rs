use actix::prelude::*;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web::web::Payload;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use actix_cors::Cors;

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

struct MyWebSocket {
    hb: Instant,
}

impl Default for MyWebSocket {
    fn default() -> Self {
        Self { hb: Instant::now() }
    }
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for MyWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(text)) => {
                println!("Received text message: {}", text);
                ctx.text(text);
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(reason)) => {
                println!("Connection closed");
                ctx.close(reason);
                ctx.stop();
            }
            _ => ctx.stop(),
        }
    }
}

impl MyWebSocket {
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("Websocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }

            ctx.ping(b"");
        });
    }
}

async fn websocket_handler(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
  let peer_addr = req.peer_addr().map(|addr| addr.to_string()).unwrap_or_else(|| "unknown".to_string());
  println!("New WebSocket connection: {}", peer_addr);
  ws::start(MyWebSocket::default(), &req, stream)
}


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
            .route("/ws/", web::get().to(websocket_handler))
            .route("/health", web::get().to(|| async { "Server is running!" }))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
