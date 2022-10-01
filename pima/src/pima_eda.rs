use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
// use pl::SeriesMethods;
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

    // let s = df.lazy()
    //         .select([col(target)])
    //         .collect()?;
    let s = Series::new(target, [0,1,0,1,0,1,0,1,1,0,1,0,1,0,0,0,0,0,1,1,1,0,0,1]);
    // let s = Series::new(target, df.select([target]));

    let target_value_counts = s.value_counts(true, true);
    println!("{:?}", target_value_counts);

    // show the correlations between all of the features of interest
    // let Glucose_BMI_corr = pearson_corr(df.lazy().select([col("Glucose")]).collect()?, 
    //                                         df.lazy().select([col("BMI")]).collect()?, 
    //                                         1);

    // df.pearson_corr("Glucose", "BMI", 1);

    // println!("{:?}", Glucose_BMI_corr);
    Ok(())
}