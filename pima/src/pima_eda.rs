use ndarray::{prelude::*, Data};
// use ndarray::arr2;
use ndarray_stats::CorrelationExt;
use color_eyre::Result;
use polars::prelude::*;
use std::io::Cursor;
use statrs::distribution::Beta;
use statrs::distribution::ContinuousCDF;
use conv::prelude::*;
// use pl::SeriesMethods;
use reqwest::blocking::Client;

pub fn pearson_correlation(data: DataFrame, feature1: &str, feature2: &str) -> Result<()> {
    let corr = data
        .select([feature1, feature2])
        .unwrap()
        .transpose()
        .unwrap()
        .to_ndarray::<Float64Type>()?
        .pearson_correlation()
        .unwrap();
    
    let r_val = corr.get((0, 1)).unwrap();
    let nrows = data.shape().0.value_as::<f64>().unwrap();
    let dist = Beta::new((nrows/ 2.0) - 1.0, (nrows/ 2.0) - 1.0).unwrap();
    let p_val = 2.0 * dist.cdf(-(r_val-1.0)/2.0);

    println!("Correlation between {} and {}:", feature1, feature2);
    println!("r-value = {:?}, p-value = {:?}", r_val, p_val);
    Ok(())
}

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
    // pearson_correlation(df, "Glucose", "BloodPressure")?;
    let feature_list = ["BloodPressure", "SkinThickness", "Insulin", "BMI", "DiabetesPedigreeFunction", "Age"];

    pearson_correlation(df, "Glucose", feature_list[3])?;

    // the for loop doesn't work with polars data frames, so need to use a different method
    // https://stackoverflow.com/questions/72372821/how-to-apply-a-function-to-multiple-columns-of-a-polars-dataframe-in-rust
    // for element in feature_list {
    //     pearson_correlation(df, "Glucose", element)?;
    // }

    Ok(())
}
