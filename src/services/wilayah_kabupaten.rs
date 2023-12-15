use crate::app_data::AppData;
use crate::services::deserialize::deserial_filter;

use actix_web::{web, HttpResponse, Responder};
use polars::prelude::*;
use polars::sql::SQLContext;
use serde_json::json;

pub async fn service(
    path: web::Path<(String, String, String)>,
    data: web::Data<AppData>,
) -> impl Responder {
    let path = path.into_inner();
    let source_type = path.0;

    let df = data.data.clone();
    let mut ctx = SQLContext::new();
    ctx.register("df", df.lazy());
    let filtered = ctx
        .execute(
            format!(
                "SELECT * FROM df WHERE length(kode_bps) = 7 AND STARTS_WITH(kode_bps, '{}{}')",
                path.1, path.2
            )
            .as_str(),
        )
        .unwrap()
        .collect();

    let df = filtered.unwrap();
    let result = deserial_filter(df);

    HttpResponse::Ok().json(json!({
        "source_type": source_type.clone(),
        "data": result
    }))
}
