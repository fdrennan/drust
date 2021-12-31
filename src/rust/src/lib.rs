//! TODO: Write crate docs

#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod drust;

use extendr_api::prelude::*;
use drust::dev::features::linear_regression;
use drust::io::dataframe_to_polars;
/// execute_lr
/// TODO execute_lf
///
/// ## Linear Regression from Rus
///
/// @export
#[extendr]
pub fn execute_lr(dataset: Robj) -> Robj {
    let df = dataframe_to_polars(&dataset);
    let response = linear_regression(&df, "cyl");
    println!("{:?}", response);
    dataset
}

#[cfg(test)]
mod tests {
    use extendr_api::prelude::*;
    use extendr_engine::start_r;
    #[test]
    fn execute_lr_works() {
        start_r();
    }
}

// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn execute_lr;
}
