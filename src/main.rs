pub mod app_data;
pub mod services;
pub mod utility;

use crate::app_data::AppData;
use actix_web::{web, App, HttpServer};
use polars::prelude::*;
use std::sync::Arc;

fn read_data() -> PolarsResult<DataFrame> {
    let mut schema = Schema::new();
    schema.with_column("kode_bps".into(), DataType::Utf8);
    schema.with_column("nama_bps".into(), DataType::Utf8);
    schema.with_column("kode_dagri".into(), DataType::Utf8);
    schema.with_column("nama_dagri".into(), DataType::Utf8);

    CsvReader::from_path("./data.csv")?
        .has_header(true)
        .with_dtypes(Some(Arc::new(schema)))
        .finish()
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let df = read_data().unwrap();
        App::new()
            .app_data(web::Data::new(AppData { data: df }))
            .service(services::service())
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
