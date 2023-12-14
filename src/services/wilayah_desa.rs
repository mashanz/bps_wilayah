use crate::app_data::AppData;
use actix_web::{web, HttpResponse, Responder};
use polars::prelude::*;
use polars::sql::SQLContext;

pub async fn service(
    path: web::Path<(String, String, String, String, String)>,
    data: web::Data<AppData>,
) -> impl Responder {
    println!("{:?}", path);
    println!("{:?}", data.data);

    let df = data.data.clone();
    let mut ctx = SQLContext::new();
    ctx.register("df", df.lazy());
    let filtered = ctx
        .execute(
            "SELECT * FROM df WHERE length(kode_bps) = 10 AND STARTS_WITH(kode_bps, '1101010001')",
        )
        .unwrap()
        .collect();

    println!("{:?}", filtered);

    HttpResponse::Ok().body("Hey there!")
}
