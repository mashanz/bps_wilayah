use crate::app_data::AppData;
use crate::services::struct_wilayah::WilayahData;
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
            format!(
                "SELECT * FROM df WHERE length(kode_bps) = 10 AND STARTS_WITH(kode_bps, '{}{}{}')",
                path.1, path.2, path.3
            )
            .as_str(),
        )
        .unwrap()
        .collect();

    let df = filtered.unwrap();
    let mut result: Vec<WilayahData> = vec![];
    let mut vec_kode_bps: Vec<String> = vec![];
    let mut vec_nama_bps: Vec<String> = vec![];
    let mut vec_kode_dagri: Vec<String> = vec![];
    let mut vec_nama_dagri: Vec<String> = vec![];

    df.iter().for_each(|row| {
        if row._field().name == "kode_bps" {
            let ref_vec_kode_bps = &mut vec_kode_bps;
            row.iter().for_each(|col| {
                // let ref_vec_kode_bps = &mut vec_kode_bps;
                ref_vec_kode_bps.push(rem_first_and_last(col.to_string().as_str()));
            });
        }
        if row._field().name == "nama_bps" {
            row.iter().for_each(|col| {
                let ref_vec_nama_bps = &mut vec_nama_bps;
                ref_vec_nama_bps.push(rem_first_and_last(col.to_string().as_str()));
            });
        }
        if row._field().name == "kode_dagri" {
            row.iter().for_each(|col| {
                let ref_vec_kode_dagri = &mut vec_kode_dagri;
                ref_vec_kode_dagri.push(rem_first_and_last(col.to_string().as_str()));
            });
        }
        if row._field().name == "nama_dagri" {
            row.iter().for_each(|col| {
                let ref_vec_nama_dagri = &mut vec_nama_dagri;
                ref_vec_nama_dagri.push(rem_first_and_last(col.to_string().as_str()));
            });
        }
    });

    for i in 0..vec_kode_bps.len() {
        let kode_bps = vec_kode_bps[i].clone();
        let nama_bps = vec_nama_bps[i].clone();
        let kode_dagri = vec_kode_dagri[i].clone();
        let nama_dagri = vec_nama_dagri[i].clone();
        let wilayah_data = WilayahData {
            kode_bps,
            nama_bps,
            kode_dagri,
            nama_dagri,
        };
        result.push(wilayah_data);
    }

    HttpResponse::Ok().json(json!({
        "source_type": source_type.clone(),
        "data": result
    }))
}
