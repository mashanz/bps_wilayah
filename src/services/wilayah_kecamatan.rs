use crate::app_data::AppData;
use crate::services::struct_wilayah::{Wilayah, WilayahData};
use crate::utility::rem_first_and_last;
use actix_web::{web, HttpResponse, Responder};
use polars::prelude::*;
use polars::sql::SQLContext;
use serde_json::json;

pub async fn service(
    path: web::Path<(String, String, String, String)>,
    data: web::Data<AppData>,
) -> impl Responder {
    let path = path.into_inner();
    let source_type = path.0;

    let df = data.data.clone();
    let mut ctx = SQLContext::new();
    ctx.register("df", df.lazy());
    let filtered = ctx
        .execute(
            "SELECT * FROM df WHERE length(kode_bps) = 10 AND STARTS_WITH(kode_bps, '1101010')",
        )
        .unwrap()
        .collect();

    println!("{:?}", filtered);

    let vec_result: Vec<WilayahData> = filtered
        .unwrap()
        .select(&["kode_bps", "nama_bps", "kode_dagri", "nama_dagri"])
        .into_iter()
        .map(|row| {
            let te = row.column("kode_bps").unwrap().get(0).unwrap();
            println!("TE === {:?}", rem_first_and_last(te.to_string().as_str()));

            let kode_bps = rem_first_and_last(
                row.column("kode_bps")
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
            let nama_bps = rem_first_and_last(
                row.column("nama_bps")
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
            let kode_dagri = rem_first_and_last(
                row.column("kode_dagri")
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
            let nama_dagri = rem_first_and_last(
                row.column("nama_dagri")
                    .unwrap()
                    .get(0)
                    .unwrap()
                    .to_string()
                    .as_str(),
            );
            WilayahData {
                kode_bps,
                nama_bps,
                kode_dagri,
                nama_dagri,
            }
        })
        .collect();

    println!("VEC RESULT === {:?}", vec_result);

    let obj = Wilayah {
        source_type: source_type.clone(),
        result: vec_result,
    };

    HttpResponse::Ok().json(obj)
}
