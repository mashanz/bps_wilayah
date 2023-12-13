use crate::app_data::AppData;
use actix_web::{web, HttpResponse, Responder};

pub async fn service(
    path: web::Path<(String, String, String)>,
    data: web::Data<AppData>,
) -> impl Responder {
    println!("{:?}", path);
    println!("{:?}", data.data);
    HttpResponse::Ok().body("Hey there!")
}
