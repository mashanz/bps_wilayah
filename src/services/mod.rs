pub mod struct_wilayah;
pub mod wilayah;
pub mod wilayah_desa;
pub mod wilayah_kabupaten;
pub mod wilayah_kecamatan;
pub mod wilayah_provinsi;

use actix_web::web;

pub fn service() -> actix_web::Scope {
    web::scope("/wilayah")
        .service(web::resource("{source_type}").route(web::get().to(wilayah::service)))
        .service(
            web::resource("{source_type}/{provinsi_id}")
                .route(web::get().to(wilayah_provinsi::service)),
        )
        .service(
            web::resource("{source_type}/{provinsi_id}/{kabupaten_id}")
                .route(web::get().to(wilayah_kabupaten::service)),
        )
        .service(
            web::resource("{source_type}/{provinsi_id}/{kabupaten_id}/{kecamatan_id}")
                .route(web::get().to(wilayah_kecamatan::service)),
        )
        .service(
            web::resource("{source_type}/{provinsi_id}/{kabupaten_id}/{kecamatan_id}/{desa_id}")
                .route(web::get().to(wilayah_desa::service)),
        )
}
