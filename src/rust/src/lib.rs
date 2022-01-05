//! TODO: Write crate docs
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod pipeline;

use extendr_api::prelude::*;

use pipeline::features::linear_regression;
use pipeline::io::dataframe_to_polars;

/// execute_lr
/// TODO execute_lf
///
/// ## Linear Regression from Rus
/// |>
/// @export
#[extendr]
pub fn execute_lr(dataset: Robj, dtypes: Robj, target: &str) -> Robj {
    println!("Dataset received on Rust");
    println!("dtypes {:?}", dtypes);
    println!("{:?}", dataset);
    let df = dataframe_to_polars(&dataset);
    // println!("{:?}", df);
    let response = linear_regression(&df, target);
    response.into_robj()
}

#[cfg(test)]
mod tests {
    use crate::execute_lr;
    use extendr_api::prelude::*;
    use extendr_engine::start_r;
    use itertools::{repeat_n, Itertools};
    use std::iter::Zip;

    #[test]
    fn dataframe() {
        test! {
            let res = data_frame!(x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], y = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
            let dtypes = r!(["integer", "integer"]);
            // TODO - Data Frame needs to parse correctly with execute_lr.
            println!("{:?}", res);
            let res = execute_lr(res, dtypes, "y");
            // println!("{:?}", res);
            assert_eq!(1, 1);
        }
    }

    #[test]
    fn scratch() {
        let elem = repeat_n(vec![[2, 3, 4]], 3);
        for x in elem {
            println!("{:?}", x);
        }
    }
}

// See corresponding C code in `entrypoint.c`.
extendr_module! {
    mod rustr;
    fn execute_lr;
}
