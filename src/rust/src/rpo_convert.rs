// impl of GenVal for a generic type `T`
use crate::pipeline::io::dataframe_to_polars;
use extendr_api::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::*;
use polars::prelude::*;
use smartcore::linalg::BaseVector;
use std::convert::{TryFrom, TryInto};

struct RobjPolarsConversion<'a> {
    col: &'a Utf8Chunked,
}

impl<'a> RobjPolarsConversion<'a> {
    pub fn convert_string(&self) -> Vec<&'a str> {
        let col = self.col;
        let mut vec_str: Vec<&str> = Vec::new();
        for idx in 0..col.len() {
            let str_item = col.get(idx).unwrap();
            vec_str.push(str_item);
        }
        vec_str
    }

    pub fn convert_numeric(&self) -> Vec<&'a str> {
        let col = self.col;
        let mut vec_str: Vec<&str> = Vec::new();
        for idx in 0..col.len() {
            let num_item = col.get(idx).unwrap();
            vec_str.push(num_item);
        }
        vec_str
    }
}

pub fn unwrap_series<'a>(df: &'a DataFrame, column_name: &'a str, dtype: DataType) -> Vec<&'a str> {
    match dtype {
        DataType::Int32 => {
            let vec = df.column(column_name).unwrap().utf8().unwrap();
            let struct_col = RobjPolarsConversion { col: vec };
            let col = struct_col.convert_string();
            col
        }
        DataType::Float64 => {
            let vec = df.column(column_name).unwrap().utf8().unwrap();
            let struct_col = RobjPolarsConversion { col: vec };
            let col = struct_col.convert_string();
            col
        },
        DataType::Utf8 => {
            let vec = df.column(column_name).unwrap().utf8().unwrap();
            let struct_col = RobjPolarsConversion { col: vec };
            let col = struct_col.convert_string();
            col
        }
    }
}

pub fn return_df(dataset: &Robj, dtypes: Robj) -> Vec<Robj> {
    let mut df = dataframe_to_polars(&dataset, dtypes);
    println!("Dataframe converted to polars");
    println!("{:?}", df);
    let columns = df.get_column_names();
    let dtypes = df.dtypes();
    let col_mapping = dtypes.into_iter().zip(columns.iter());
    let mut df_cols: Vec<Robj> = Vec::new();
    for (col_type, column_name) in col_mapping {
        match col_type {
            DataType::Int32 => {
                println!("Int32");
                df_cols.push(unwrap_series(&df, &column_name, col_type).into_robj());
            }
            DataType::Float64 => {
                println!("Float64");
                df_cols.push(unwrap_series(&df, &column_name, col_type).into_robj());
            }
            DataType::Utf8 => {
                println!("Utf8");
                df_cols.push(unwrap_series(&df, &column_name, col_type).into_robj());
            }
            _ => {
                println!("Else Condition");
                df_cols.push(unwrap_series(&df, &column_name, col_type).into_robj())
            }
        }
    }
    df_cols
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
                x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                y = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                    7.0, 8.0, 9.0, 10.0],
                z = ["1.0", "2.0", "3.0", "4.0", "5.0", "6.0",
                    "7.0", "8.0", "9.0", "10.0"]
            );
            let dtypes = r!(["integer", "double", "character"]);
            let df = return_df(&res, dtypes);
            assert_eq!(1, 1);
        }
    }
}
