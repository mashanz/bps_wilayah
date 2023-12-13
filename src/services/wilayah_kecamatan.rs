use actix_web::{web, HttpResponse, Responder};

pub async fn service(path: web::Path<(String, String, String, String)>) -> impl Responder {
    println!("{:?}", path);
    HttpResponse::Ok().body("Hey there!")
}
