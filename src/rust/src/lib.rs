//! TODO: Write crate docs

// #![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
use extendr_api::prelude::*;

mod pipeline;

use pipeline::dev::features::linear_regression;
use pipeline::dev::io::read_csv;

use time::PreciseTime;

use ndarray::prelude::*;
use ndarray::{Data, ShapeBuilder};
use ndarray::arr2;
/// execute_lr
/// TODO execute_lf
///
/// ## Linear Regression from Rust
///
/// @export
#[extendr]
pub fn execute_lr(dataset: Robj, col_names: Robj, types: Robj) -> Robj {
    fn parse_robj(dataset: Robj) -> Vec<&'static str> {
        let mut vect = Vec::new();
        for x in dataset.as_str_iter() {
            x.for_each(|y| vect.push(y))
        }
        vect
    }
    let data = parse_robj(dataset);
    let col_names = parse_robj(col_names);
    let types = parse_robj(types);
    let chunk_len = data.len()/col_names.len();
    rprintln!("{:?}", chunk_len);

    let mut arr = Array1::<i32>::default(data.len());


    let mut data_iter = data.chunks_exact(chunk_len);
    for col in col_names {
        let data_next = data_iter.next().unwrap();
        println!("{:?}", col);
        println!("{:?}", data_next);
    }
    println!("{:?}", arr);

    data.into_robj()
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
