use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

use crate::firebase::{save_message, get_messages};

#[derive(Deserialize)]
struct SendMessageRequest {
    room: String,
    user: String,
    content: String,
}

async fn send_message(data: web::Json<SendMessageRequest>) -> impl Responder {
    let result = save_message(&data.room, &data.user, &data.content).await;

    match result {
        Ok(_) => HttpResponse::Ok().json("Message envoyé avec succès"),
        Err(e) => {
            eprintln!("Erreur lors de l'envoi du message: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Erreur: {}", e))
        },
    }
}

async fn get_messages_handler(room: web::Path<String>) -> impl Responder {
    let result = get_messages(&room).await;

    match result {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(e) => {
            eprintln!("Erreur lors de la récupération des messages: {:?}", e);
            HttpResponse::InternalServerError().body(format!("Erreur: {}", e))
        },
    }
}

pub fn init(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/send-message")
            .route(web::post().to(send_message))
    )
    .service(
        web::resource("/get-messages/{room}")
            .route(web::get().to(get_messages_handler))
    );
}
