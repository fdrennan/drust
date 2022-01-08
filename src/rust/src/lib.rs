//! TODO: Write crate docs
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use extendr_api::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::*;
use polars::prelude::*;
use smartcore::linalg::BaseVector;
use std::convert::{TryFrom, TryInto};

pub fn dataframe_to_polars(dataset: &Robj, dtypes: Robj) -> DataFrame {
    let col_names = dataset.names().unwrap();
    let dtypes = dtypes.as_str_vector().unwrap();
    let it = col_names.into_iter().zip(dtypes.iter());

    let mut df_cols: Vec<Series> = Vec::new();
    for (col, dtype) in it.into_iter() {
        let message = format!("Converting {} to {}", col, dtype);
        prettycli::info(message.as_str());
        match *dtype {
            "integer" => {
                let col_data = dataset.dollar(col).unwrap();
                let col_data = col_data.as_integer_vector().unwrap();
                let s = Series::new(col, col_data);
                df_cols.push(s)
            }
            "double" => {
                let col_data = dataset.dollar(col).unwrap();
                let col_data = col_data.as_real_vector().unwrap();
                let s = Series::new(col, col_data);
                df_cols.push(s)
            }
            _ => {
                let col_data = dataset.dollar(col).unwrap();
                let col_data = col_data.as_string_vector().unwrap();
                let s = Series::new(col, col_data);
                df_cols.push(s)
            }
        };
    }

    println!("df_cols = {:?}", df_cols);
    let df = DataFrame::new(df_cols);
    df.unwrap()
}
#
pub fn return_df(dataset: &Robj, dtypes: Robj) -> Robj {
    let mut df = dataframe_to_polars(&dataset, dtypes);
    let columns = df.get_column_names();
    let dtypes = df.dtypes();
    let col_mapping = dtypes.into_iter().zip(columns.iter());
    let mut data_frame_columns: Vec<Robj> = Vec::new();
    for (col_type, column_name) in col_mapping {
        match col_type {
            DataType::Utf8 => {
                let old_vec = df.column(column_name).unwrap().utf8().unwrap();
                let mut new_vec: Vec<&str> = Vec::new();
                for idx in 0..old_vec.len() {
                    new_vec.push(old_vec.get(idx).unwrap());
                }
                data_frame_columns.push(new_vec.into_robj());
            }
            DataType::Int32 => {
                let old_vec: &Int32Chunked = df.column(column_name).unwrap().i32().unwrap();
                let mut new_vec: Vec<i32> = Vec::new();
                for idx in 0..old_vec.len() {
                    new_vec.push(old_vec.get(idx).unwrap());
                }
                data_frame_columns.push(new_vec.into_robj());
            }
            DataType::Float64 => {
                let old_vec: &Float64Chunked = df.column(column_name).unwrap().f64().unwrap();
                let mut new_vec: Vec<f64> = Vec::new();
                for idx in 0..old_vec.len() {
                    new_vec.push(old_vec.get(idx).unwrap());
                }
                data_frame_columns.push(new_vec.into_robj());
            }
            _ => {
                panic!("Else condition hit");
            }
        }
    }
    let mut raw_df: Vec<(&str, Robj)> = Vec::new();
    let raw_df_data = columns.into_iter().zip(data_frame_columns);
    for (name, vec) in raw_df_data.into_iter() {
        raw_df.push((name, vec));
    }
    let new_robj = r!(List::from_pairs(raw_df))
        .set_class(&["data.frame"])
        .unwrap();
    println!("{:?}", new_robj);
    new_robj
}

#[cfg(test)]
mod tests {
    use crate::return_df;
    use extendr_api::prelude::*;
    use extendr_engine::start_r;
    use std::convert::TryInto;
    use std::iter::Zip;

    #[test]
    fn convert_return() {
        test! {
            let res = data_frame!(
                z = ["1.0", "2.0", "3.0", "4.0", "5.0", "6.0",
                    "7.0", "8.0", "9.0", "10.0"],
                x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                y = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                    7.0, 8.0, 9.0, 10.0]
            );
            let dtypes = r!(["character", "integer", "double"]);
            let df = return_df(&res, dtypes);
            println!("need {:?}", res);
            assert_eq!(1, 1);
        }
    }
}

//
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn return_df;
}
