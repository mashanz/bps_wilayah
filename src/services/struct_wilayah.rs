use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct WilayahData {
    pub kode_bps: String,
    pub nama_bps: String,
    pub kode_dagri: String,
    pub nama_dagri: String,
}

#[derive(Serialize, Debug)]
pub struct Wilayah {
    pub source_type: String,
    pub result: Vec<WilayahData>,
}
