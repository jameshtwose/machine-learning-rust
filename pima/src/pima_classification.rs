use color_eyre::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;
use std::io::Cursor;

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
        .unwrap()
        .into_raw_vec();

    let y = df
        .select([target])
        .unwrap()
        .to_ndarray::<Float64Type>()
        .unwrap()
        .into_raw_vec();

    // println!("x: {:?}", x);
    // println!("y: {:?}", y);

    // fit the model
    // let model = LogisticRegression::fit(&x, &y, Default::default()).unwrap();

    Ok(())
}
