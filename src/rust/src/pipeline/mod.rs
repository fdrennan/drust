pub mod io {
    use extendr_api::prelude::*;
    use polars::frame::DataFrame;
    use polars::prelude::*;

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
}

pub mod features {
    use extendr_api::prelude::*;
    use polars::frame::DataFrame;
    use polars::prelude::Float64Type;
    use polars::prelude::Result as PolarResult;
    use smartcore::linalg::naive::dense_matrix::DenseMatrix;
    use smartcore::linalg::BaseMatrix;
    use smartcore::linear::linear_regression::LinearRegression;
    use smartcore::metrics::mean_squared_error;
    use smartcore::model_selection::train_test_split;
    use std::convert::TryFrom;
    pub fn feature_and_target(
        in_df: &DataFrame,
        target_column: &str,
    ) -> (PolarResult<DataFrame>, PolarResult<DataFrame>) {
        let target = in_df.select(target_column);
        let predictors = in_df.drop(target_column);
        let predictor_names = predictors.as_ref().unwrap().get_column_names();
        println!("Predicting on {:?}", target_column);
        let predictor_names = predictor_names
            .into_iter()
            .map(|i| i.to_string() + ", ")
            .collect::<std::string::String>();
        println!("...using features {:?}", predictor_names);
        (predictors, target)
    }

    pub fn convert_features_to_matrix(in_df: &DataFrame) -> Result<DenseMatrix<f64>> {
        let nrows = in_df.height();
        let ncols = in_df.width();

        let features_res = in_df.to_ndarray::<Float64Type>().unwrap();
        let mut xmatrix: DenseMatrix<f64> = BaseMatrix::zeros(nrows, ncols);
        let mut col: u32 = 0;
        let mut row: u32 = 0;

        for val in features_res.iter() {
            let m_row = usize::try_from(row).unwrap();
            let m_col = usize::try_from(col).unwrap();
            xmatrix.set(m_row, m_col, *val);
            if m_col == ncols - 1 {
                row += 1;
                col = 0;
            } else {
                col += 1;
            }
        }
        Ok(xmatrix)
    }

    pub fn split_df(features: &DataFrame, target: &DataFrame) -> (DenseMatrix<f64>, Vec<f64>) {
        let x_matrix = convert_features_to_matrix(features);
        let target_array = target.to_ndarray::<Float64Type>().unwrap();

        let mut y: Vec<f64> = Vec::new();
        for val in target_array.iter() {
            y.push(*val);
        }

        (x_matrix.unwrap(), y)
    }

    pub fn linear_regression(df: &DataFrame, target: &str) -> Vec<f64> {
        let (features, target) = feature_and_target(df, target);
        let (x_matrix, y) = split_df(features.as_ref().unwrap(), target.as_ref().unwrap());
        let (x_train, x_test, y_train, y_test) = train_test_split(&x_matrix, &y, 0.3, true);
        let linear_regression =
            LinearRegression::fit(&x_train, &y_train, Default::default()).unwrap();
        let predictions = linear_regression.predict(&x_test).unwrap();
        let mse = mean_squared_error(&y_test, &predictions);
        println!("MSE: {:?}", mse);
        predictions
    }
}
