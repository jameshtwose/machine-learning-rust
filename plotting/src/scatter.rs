use color_eyre::Result;
use polars::prelude::*;
use reqwest::blocking::Client;
use std::io::Cursor;

use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::{Bar, NamedColor, Plot, Rgb, Rgba, Scatter};

pub fn run() -> Result<()> {
    println!("Exploratory Data Analysis - Scatter Plot");
    println!("=========================");

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

    let x: Vec<f64> = df
        .column("bill_length_mm")
        .unwrap()
        .f64()?
        .into_no_null_iter()
        .collect();

    let y: Vec<f64> = df
        .column("bill_depth_mm")
        .unwrap()
        .f64()?
        .into_no_null_iter()
        .collect();

    // simple_scatter_plot(x, y, true, false);

    Ok(())
}
