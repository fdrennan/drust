//! TODO: Write crate docs

// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use extendr_api::prelude::*;

mod pipeline;

use pipeline::features::linear_regression;
use pipeline::io::read_csv;

use time::PreciseTime;
/// execute_lr
/// TODO execute_lf
/// @export
#[extendr]
pub fn execute_lr(file_path: &str, target: &str) -> Robj {
    let start = PreciseTime::now();
    // whatever you want to do
    let df = read_csv(&file_path).unwrap();
    let predictions = linear_regression(&df, target);
    let end = PreciseTime::now();
    rprintln!("{} seconds elapsed...", start.to(end));
    predictions.into_robj()
}

// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn execute_lr;
}
