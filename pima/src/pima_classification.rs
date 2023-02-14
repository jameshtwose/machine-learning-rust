use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;
use smartcore::linear::logistic_regression::LogisticRegression;
use smartcore::metrics::accuracy;

pub fn run() -> Result<()> { 

    println!("Classification");
    println!("==============");
    
    // specify the URL where the csv is located
    let csv_url = "https://raw.githubusercontent.com/jameshtwose/example_deliverables/main/classification_examples/pima_diabetes/diabetes.csv";
    
    // read in the data as a vector using the reqwest Client
    let data: Vec<u8> = Client::new()
        .get(csv_url)
        .send()?
        .text()?
        .bytes()
        .collect();
    
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

    let X = df
        .drop(target)?
        .to_ndarray::<Float64Type>()?;

    let y = df.select([target])
        .unwrap()
        .to_ndarray::<Float64Type>()?;

    println!("X: {:?}", X);
    println!("y: {:?}", y);

    Ok(())
}