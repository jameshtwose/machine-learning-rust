use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;

use itertools_num::linspace;
use plotly::common::{
    ColorScale, 
    ColorScalePalette, 
    DashType, 
    Fill, 
    Font, 
    Line, 
    LineShape, 
    Marker, 
    Mode, 
    Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};

pub fn run() -> Result<()> {
    
    println!("Exploratory Data Analysis");
    println!("=========================");
    
    // specify the URL where the csv is located
    let csv_url = "https://raw.githubusercontent.com/mwaskom/seaborn-data/master/penguins.csv";
    
    // read in the data as a vector using the reqwest Client
    let data: Vec<u8> = Client::new()
        .get(csv_url)
        .send()?
        .text()?
        .bytes()
        .collect();
    
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
    
    // print the head of the data frame (first x rows)
    // the amount of rows and columns
    // and the descriptive statistics of the columns
    println!("Head of the Data Frame:");
    println!("{:?}", df.head(Some(3)));
    println!("Shape of the whole Data Frame:");
    println!("{:?}", df.shape());
    println!("Amount of missings per column before removal:");
    println!("{:?}", pre_df.null_count());
    println!("Amount of missings per column after removal:");
    println!("{:?}", df.null_count());

    // fn simple_scatter_plot(x: ChunkedArray<Float64Type>, y: ChunkedArray<Float64Type>, show: bool, saveimg: bool) {
    fn simple_scatter_plot(x: Vec<f64>, y: Vec<f64>, show: bool, saveimg: bool) {

        let trace = Scatter::new(x, y).mode(Mode::Markers);
        // let trace = Scatter::from_array(x, y).mode(Mode::Markers);
        let mut plot = Plot::new();
        plot.add_trace(trace);
        if show {
            plot.show();
        }
        if saveimg {
            plot.to_html("images/scatter_plot.html");
        }
    }
    // let n: usize = 100;
    // let x: Vec<f64> = linspace(0., 10., n).collect();
    // let y: Vec<f64> = x.iter().map(|x| x.sin()).collect();

    let x_array: ChunkedArray<Float64Type> = df["bill_length_mm"].f64()?.into_iter().collect();
    let y_array: ChunkedArray<Float64Type> = df["bill_depth_mm"].f64()?.into_iter().collect();

    // // let x = vec![];
    // for i in x_array.into_no_null_iter() {
    //     println!("{}", i);
    //     // x.append(i);
    // }

    // // let y = vec![];
    // for i in y_array.into_no_null_iter() {
    //     println!("{}", i);
    //     // y.append(i);
    // }

    // bill_length_mm
    let x: Vec<f64> = vec![39.1,
                        39.5,
                        40.3,
                        36.7,
                        39.3,
                        38.9,
                        39.2,
                        41.1,
                        38.6,
                        34.6,
                        36.6,
                        38.7];

    // bill_depth_mm
    let y: Vec<f64> = vec![18.7,
                            17.4,
                            18.0,
                            19.3,
                            20.6,
                            17.8,
                            19.6,
                            17.6,
                            21.2,
                            21.1,
                            17.8,
                            19.0];

    // let x: Vec<f64> = df["bill_length_mm"].i64()?.into_iter().collect();
    // let x_array: ChunkedArray<Float64Type> = df["bill_length_mm"].f64()?.into_iter().collect();
    // let y_array: ChunkedArray<Float64Type> = df["bill_depth_mm"].f64()?.into_iter().collect();

    // let x = x_array;

    // let y: Vec<f64> = x.iter().map(|x| x.sin()).collect();

    // let x_mask: ChunkedArray<BooleanType> = df["bill_length_mm"].is_not_null();
    // let x_array = df.select([x_mask]);
    // let x = vec![x_array];

    // let x = df["bill_length_mm"].f64()?.into_iter().collect();
    simple_scatter_plot(x, y, true, true);

    Ok(())
}