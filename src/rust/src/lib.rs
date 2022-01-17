#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use extendr_api::prelude::*;
use extendr_api::RType::Symbol;
use polars::frame::DataFrame;
use polars::prelude::*;
use polars::prelude::*;
use smartcore::linalg::BaseVector;
use std::convert::{TryFrom, TryInto};

pub fn polars_to_dataframe(df: DataFrame) -> Robj {
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

    let nrows = data_frame_columns.first().unwrap().len();
    let nrows = i32::try_from(nrows).unwrap() + 1;
    let mut raw_df: Vec<(&str, Robj)> = Vec::new();
    let raw_df_data = columns.into_iter().zip(data_frame_columns);
    for (name, vec) in raw_df_data.into_iter() {
        raw_df.push((name, vec));
    }
    let new_dataset = r!(List::from_pairs(raw_df))
        .set_class(&["data.frame"])
        .unwrap();
    let new_dataset = new_dataset.set_attrib("row.names", r![1..nrows]).unwrap();
    new_dataset
}
pub fn dataframe_to_polars(dataset: &Robj, dtypes: Robj) -> DataFrame {
    let col_names = dataset.names().unwrap();
    let dtypes = dtypes.as_str_vector().unwrap();
    let it = col_names.into_iter().zip(dtypes.iter());
    let mut df_cols: Vec<Series> = Vec::new();
    for (col, dtype) in it.into_iter() {
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

    let df = DataFrame::new(df_cols);
    df.unwrap()
}

/// return_df
/// TODO execute_lf
///
/// ## Linear Regression from Rust
///
/// @export
#[extendr]
pub fn return_df(dataset: Robj, dtypes: Robj) -> Robj {
    println!("Received df {:?}", dataset);
    let polars_df = dataframe_to_polars(&dataset, dtypes);
    rprintln!("{:?}", polars_df);
    let new_dataset = polars_to_dataframe(polars_df);
    new_dataset
}

// #[cfg(test)]
// mod tests {
//     use crate::dataframe_to_polars;
//     use crate::polars_to_dataframe;
//     use crate::return_df;
//     use extendr_api::prelude::*;
//     use extendr_engine::start_r;
//     use std::convert::TryInto;
//     use std::iter::Zip;
//     #[test]
//     fn convert_return() {
//         test! {
//                 let dataset = data_frame!(
//                     z = ["1.0", "2.0", "3.0", "4.0", "5.0", "6.0",
//                         "7.0", "8.0", "9.0", "10.0"],
//                     x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
//                     y = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
//                         7.0, 8.0, 9.0, 10.0]
//                 );
//                 let dtypes = r!(["character", "integer", "double"]);
//             let mut polars_df = dataframe_to_polars(&dataset, dtypes);
//         let new_dataset = polars_to_dataframe(polars_df);
//             println!("{:?}", dataset);
//         }
//     }
// }

//
// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod redpul;
    fn return_df;
}
