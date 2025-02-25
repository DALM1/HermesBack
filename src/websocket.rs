use actix::prelude::*;
use actix_web::{Error, HttpRequest, HttpResponse};
use actix_web::web::Payload;
use actix_web_actors::ws;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Message {
    Text { user: String, content: String },
    Join { user: String, room: String },
    Leave { user: String, room: String },
}

pub struct MyWebSocket {
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
                self.handle_message(&text, ctx);
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
    fn handle_message(&mut self, msg: &str, ctx: &mut ws::WebsocketContext<Self>) {
        match serde_json::from_str::<Message>(msg) {
            Ok(message) => match message {
                Message::Text { user, content } => {
                    println!("{}: {}", user, content);
                    ctx.text(format!("{}: {}", user, content));
                }
                Message::Join { user, room } => {
                    println!("{} joined room {}", user, room);
                }
                Message::Leave { user, room } => {
                    println!("{} left room {}", user, room);
                }
            },
            Err(e) => {
                println!("Failed to parse message: {:?}", e);
            }
        }
    }

    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                println!("WebSocket Client heartbeat failed, disconnecting!");
                ctx.stop();
                return;
            }
            ctx.ping(b"");
        });
    }
}

pub async fn websocket_handler(req: HttpRequest, stream: Payload) -> Result<HttpResponse, Error> {
    ws::start(MyWebSocket::default(), &req, stream)
}
