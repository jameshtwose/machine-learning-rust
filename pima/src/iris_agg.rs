use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;

pub fn run() -> Result<()> { 
    // let csv_url = "https://raw.githubusercontent.com/jameshtwose/example_deliverables/main/classification_examples/pima_diabetes/diabetes.csv"
    let csv_url = "https://j.mp/iriscsv";
    let data: Vec<u8> = Client::new()
        .get(csv_url)
        .send()?
        .text()?
        .bytes()
        .collect();

    let df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?
        .lazy()
        // .describe()
        .collect()?
        ;

    println!("{:?}", df.head(Some(3)));
    println!("{:?}", df.dtypes());
    println!("{:?}", df.describe(None));

    Ok(())
}