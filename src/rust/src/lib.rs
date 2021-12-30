//! TODO: Write crate docs

// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use extendr_api::prelude::*;
use polars::prelude::*;
mod pipeline;

fn parse_robj(dataset: &Robj) -> Vec<f64> {
    let mut vect = Vec::new();
    for x in dataset.as_real_iter() {
        x.for_each(|y| vect.push(*y))
    }
    vect
}

use pipeline::dev::features::linear_regression;
use pipeline::dev::io::read_csv;

use time::PreciseTime;

use ndarray::arr2;
use ndarray::prelude::*;
use ndarray::{Data, ShapeBuilder};
/// execute_lr
/// TODO execute_lf
///
/// ## Linear Regression from Rust
///
/// @export
#[extendr]
pub fn execute_lr(data: Robj, col_names: Robj, types: Robj, dataset: Robj) -> Robj {
    let col_names = dataset.names().unwrap();
    let mut df_cols: Vec<Series> = Vec::new();
    for col in col_names {
        let col_data = dataset
            .dollar(col)
            .unwrap()
            .as_real_vector()
            .unwrap();
        let s = Series::new(col, col_data);
        df_cols.push(s)
    }
    let df = DataFrame::new(df_cols);
    println!("{:?}", df);
    types
}

#[cfg(test)]
mod tests {
    use extendr_api::prelude::*;
    use extendr_engine::start_r;
    #[test]
    fn execute_lr_works() {
        start_r();
        // use super::execute_lr;
        // let predictions = execute_lr("mtcars.csv", "cyl");
        // let predlen = predictions.len();
        // assert_eq!(9, predlen)
    }
}

// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn execute_lr;
}
