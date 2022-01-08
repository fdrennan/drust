//! TODO: Write crate docs
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
mod pipeline;
mod rpo_convert;

use extendr_api::prelude::*;
use polars::frame::DataFrame;
use polars::prelude::*;
use rpo_convert::return_df;
use smartcore::linalg::BaseVector;

#[cfg(test)]
mod tests {
    use crate::return_df;
    use extendr_api::prelude::*;
    use std::convert::TryInto;
    use std::iter::Zip;
    #[test]
    fn df_to_polars_to_df_check() {
        test! {
            // Start with data.frame
            let res = data_frame!(
                x = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10],
                y = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
                    7.0, 8.0, 9.0, 10.0],
                z = ["1.0", "2.0", "3.0", "4.0", "5.0", "6.0",
                    "7.0", "8.0", "9.0", "10.0"]
            );
            let dtypes = r!(["integer", "double", "character"]);
            // Get data.frame back
            let res = return_df(&res, dtypes);
            println!("res is {:?}", res);
            assert_eq!(1, 1);
        }
    }
}
//
// // See corresponding C code in `entrypoint.c`.
// extendr_module! {
//     mod rustr;
//     fn return_df;
// }
