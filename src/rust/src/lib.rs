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
pub fn execute_lr(data: Robj, col_names: Robj, types: Robj) -> Robj {
    let data = data.as_real_vector().unwrap();
    let colnames = col_names.as_str_vector().unwrap();
    let mut cols_iterator = data
        .chunks_exact(col_names.len())
        .into_iter();
    let mut vec_df: Vec<Series> = Vec::new();
    for key in colnames {
        let value = cols_iterator.next().unwrap();
        println!("{:?}, {:?}", key, value);
        let s = Series::new(key, value);
        vec_df.push(s);
    }
    let df = DataFrame::new(vec_df);
    println!("{:?}", df);
    types
    // let start = PreciseTime::now();
    // println!("{:?}", dataset);
    // whatever you want to do

    // let df = read_csv(&file_path).unwrap();
    // let predictions = linear_regression(&df, target);
    // let end = PreciseTime::now();
    // rprintln!("{} seconds elapsed...", start.to(end));
    // predictions.into_robj()
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
