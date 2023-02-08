use color_eyre::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;
use std::ops::Index;
use std::convert::TryFrom;

use plotly::common::{ColorScale, ColorScalePalette, Title};
use plotly::contour::Contours;
use plotly::{Contour, HeatMap, Layout, Plot};

pub fn run() -> Result<()> {
    println!("Exploratory Data Analysis - Heatmap");
    println!("===================================");

    // define the heatmap function
    fn basic_heat_map(x: Vec<i32>, y: Vec<i32>, z: Vec<i32>, show: bool, saveimg: bool) {
        
        let input = vec![x, y, z];
        let trace = HeatMap::new_z(input);
        let mut plot = Plot::new();
        plot.add_trace(trace);
        if show {
            plot.show();
        }
        if saveimg {
            plot.to_html("images/heatmap.html");
        }
    }

    
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
    let df = pre_df.filter(&mask)?;

    let x = df.get_column_names();
    let (y, _) = df.shape();
    let new_vec: Vec<usize> = (0..y).collect();

    println!("{:?}", new_vec);

    // let x = vec![1, 20, 30]; 
    // let y = vec![20, 1, 60];
    // let z = vec![30, 60, 1];

    // basic_heat_map(x, y, z, true, true);

    Ok(())
}
