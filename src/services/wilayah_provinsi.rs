use crate::app_data::AppData;
use actix_web::{web, HttpResponse, Responder};
use polars::prelude::*;
use polars::sql::SQLContext;

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

    let df = data.data.clone();
    let mut ctx = SQLContext::new();
    ctx.register("df", df.lazy());
    let filtered = ctx
        .execute("SELECT * FROM df WHERE length(kode_bps) = 4 AND STARTS_WITH(kode_bps, '11')")
        .unwrap()
        .collect();

    println!("{:?}", filtered);

    HttpResponse::Ok().body("Hey there!")
}
