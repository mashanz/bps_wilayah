use serde::Serialize;

#[derive(Serialize)]
pub struct WilayahData {
    pub kode_bps: String,
    pub nama_bps: String,
    pub kode_dagri: String,
    pub nama_dagri: String,
}

#[derive(Serialize)]
pub struct Wilayah {
    pub source_type: String,
    pub result: Vec<WilayahData>,
}
