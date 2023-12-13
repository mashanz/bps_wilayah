use crate::app_data::AppData;
use crate::services::struct_wilayah::{Wilayah, WilayahData};
use actix_web::{web, HttpResponse, Responder};
use polars::prelude::*;

pub async fn service(path: web::Path<String>, data: web::Data<AppData>) -> impl Responder {
    let path = path.into_inner();
    let source_type = path;
    let obj = Wilayah {
        source_type: source_type.clone(),
        result: vec![WilayahData {
            kode_bps: "1".to_string(),
            nama_bps: "Kabupaten".to_string(),
            kode_dagri: "1".to_string(),
            nama_dagri: "Kabupaten".to_string(),
        }],
    };

    let df = data.data.clone().lazy();
    let filtered = df.filter(col("kode_bps"));

    // Materialize the LazyFrame to see the resulting DataFrame
    let result_df = filtered.collect();
    HttpResponse::Ok().json(obj)
}
