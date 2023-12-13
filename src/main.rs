pub mod app_data;
pub mod services;

use crate::app_data::AppData;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use polars::prelude::*;

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .app_data(web::Data::new(AppData {
                data: DataFrame::default(),
            }))
            .route("/hey", web::get().to(manual_hello))
            .service(services::service())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
