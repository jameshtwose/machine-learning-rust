use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;

pub fn run() -> Result<()> { 
    let csv_url = "https://raw.githubusercontent.com/jameshtwose/example_deliverables/main/classification_examples/pima_diabetes/diabetes.csv";
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
        .collect()?;

    let target = "Outcome";
    
    println!("Head of the Data Frame:");
    println!("{:?}", df.head(Some(3)));
    println!("Shape of the whole Data Frame:");
    println!("{:?}", df.shape());
    println!("Data Frame description:");
    println!("{:?}", df.describe(None));

    Ok(())
}