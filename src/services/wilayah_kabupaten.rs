use actix_web::{HttpResponse, Responder};

pub async fn service() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}
