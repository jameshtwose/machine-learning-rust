use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;

pub fn run() -> Result<()> { 
    
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
    
    // print the head of the data frame (first x rows)
    // the amount of rows and columns
    // and the descriptive statistics of the columns
    println!("Head of the Data Frame:");
    println!("{:?}", df.head(Some(3)));
    println!("Shape of the whole Data Frame:");
    println!("{:?}", df.shape());
    println!("Data Frame description:");
    println!("{:?}", df.describe(None));

    let target_value_counts = df
                                        .lazy()
                                        .select([col(target).value_counts(true)])
                                        .collect()?
                                        ;
    println!("{:?}", target_value_counts);

    Ok(())
}