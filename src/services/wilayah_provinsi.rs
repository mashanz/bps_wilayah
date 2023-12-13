use crate::app_data::AppData;
use actix_web::{web, HttpResponse, Responder};

pub async fn service(
    path: web::Path<(String, String)>,
    data: web::Data<AppData>,
) -> impl Responder {
    let path = path.into_inner();
    let source_type = path.0;
    let provinsi_id = path.1;
    println!("{:?}", source_type);
    println!("{:?}", provinsi_id);
    println!("{:?}", data.data);
    HttpResponse::Ok().body("Hey there!")
}
