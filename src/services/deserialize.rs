use crate::services::struct_wilayah::WilayahData;
use crate::utility::rem_first_and_last;
use polars::prelude::*;

pub fn deserial_filter(df: DataFrame) -> Vec<WilayahData> {
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
    result
}
