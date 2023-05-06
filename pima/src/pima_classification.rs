use color_eyre::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;
use linfa::{prelude::*, dataset};
// use linfa_logistic::error::Result;
use linfa_logistic::LogisticRegression;

pub fn run() -> Result<()> {
    println!("Classification");
    println!("==============");

    // specify the URL where the csv is located
    let csv_url = "https://raw.githubusercontent.com/jameshtwose/example_deliverables/main/classification_examples/pima_diabetes/diabetes.csv";

    // read in the data as a vector using the reqwest Client
    let data: Vec<u8> = Client::new().get(csv_url).send()?.text()?.bytes().collect();

    // convert the data to a polars data frame
    let df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;

    // specify the column name of the target
    let target = "Outcome";

    let mask_default_values = true;

    if mask_default_values {
        let tmp_df: DataFrame = df
        .drop(target)?
        // .replace_all(0, "")?
        ;
        println!("{:?}", tmp_df.head(Some(1)));
    }

    let x = df
        .drop(target)?
        .to_ndarray::<Float64Type>()
        .unwrap();
        // .into_raw_vec()

    let y = df
        .select([target])
        .unwrap()
        .to_ndarray::<Float64Type>()
        .unwrap();
        // .into_raw_vec()

    // let x_mat = DenseMatrix::from_2d_array(x);

    // println!("x: {:?}", x);
    // println!("y: {:?}", y);

    // fit the model
    // let model = LogisticRegression::fit(&x, &y, Default::default()).unwrap();
    let data: DatasetBase<ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 2]>>, ndarray::ArrayBase<ndarray::OwnedRepr<f64>, ndarray::Dim<[usize; 1]>>> = DatasetBase::new(x, y);
    // println!("data: {:?}", data);
    let model = LogisticRegression::default().fit(&data).unwrap();

    Ok(())
}
