use ndarray::prelude::*;
// use ndarray::arr2;
use ndarray_stats::CorrelationExt;
use color_eyre::Result;
use polars::prelude::*;
use std::io::Cursor;
// use pl::SeriesMethods;
use reqwest::blocking::Client;

pub fn run() -> Result<()> {
    println!("Exploratory Data Analysis");
    println!("=========================");

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

    // print the head of the data frame (first x rows)
    // the amount of rows and columns
    // and the descriptive statistics of the columns
    println!("Head of the Data Frame:");
    println!("{:?}", df.head(Some(3)));
    println!("Shape of the whole Data Frame:");
    println!("{:?}", df.shape());
    println!("Data Frame description:");
    println!("{:?}", df.describe(None));
    println!("Value counts of the target:");
    println!(
        "{:?}",
        df.column(target).unwrap().clone().value_counts(true, true)
    );

    // calculate the correlation between the target and the other columns
    let Glucose_BMI_corr = df
        .select(["Glucose", "BMI"])
        .unwrap()
        .to_ndarray::<Float64Type>()?
        // .unwrap()
        .pearson_correlation();

    let tmp = df
        .select(["Glucose", "BMI"])
        .unwrap()
        .to_ndarray::<Float64Type>()?;

    let tmp2 = Array::from_shape_vec((2, 768).f(), vec![tmp])?;

    // let a = arr2(&[[1., 3., 5.],
    //     [2., 4., 6.]]);
    // let corr = a.pearson_correlation().unwrap();

    println!("{:?}", tmp2);
    // println!("{:?}", Glucose_BMI_corr);
    Ok(())
}
