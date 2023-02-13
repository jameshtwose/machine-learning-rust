use color_eyre::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use std::convert::TryFrom;
use std::io::Cursor;
use std::ops::Index;

use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};

pub fn run() -> Result<()> {
    println!("Exploratory Data Analysis - Heatmap");
    println!("===================================");

    // specify the URL where the csv is located
    let csv_url = "https://raw.githubusercontent.com/mwaskom/seaborn-data/master/penguins.csv";

    // read in the data as a vector using the reqwest Client
    let data: Vec<u8> = Client::new().get(csv_url).send()?.text()?.bytes().collect();

    // convert the data to a polars data frame
    let pre_df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;

    // create a mask of boolean (non null) values
    let mask = pre_df.column("sex")?.is_not_null();
    // apply the filter on a DataFrame
    let df: DataFrame = pre_df.filter(&mask)?;

    // the df.get_column_names() gives a not live long enough error in conjunction with the y creation

    // let x = df.get_column_names();
    // let (y_rows, _) = df.shape();
    // let y: Vec<usize> = (0..y_rows).collect();

    // println!("{:?}", x);

    let x = vec![0.5, 1.5, 2.5];
    let y: Vec<&str> = vec!["day 1", "day 2", "day 3"];
    // let y = vec![0.5, 1.5, 2.5];
    let z = vec![vec![10.8, 50.6, 1.3], vec![20.8, 60.6, 2.3], vec![30.8, 70.6, 3.3]];

    let trace = HeatMap::new(x, y, z);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let saveimg = false;
    let show = true;
    
    if saveimg {
        plot.to_html("images/heatmap.html");
    }
    if show {
        plot.show();
    }

    Ok(())
}
