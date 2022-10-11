use std::io::Cursor;
use color_eyre::{Result};
use polars::prelude::*;
use reqwest::blocking::Client;

use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
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
    let df = CsvReader::new(Cursor::new(data))
        .has_header(true)
        .finish()?
        .lazy()
        .collect()?;
    
    // print the head of the data frame (first x rows)
    // the amount of rows and columns
    // and the descriptive statistics of the columns
    println!("Head of the Data Frame:");
    println!("{:?}", df.head(Some(3)));
    println!("Shape of the whole Data Frame:");
    println!("{:?}", df.shape());

    fn simple_scatter_plot(show: bool, saveimg: bool) {
        let n: usize = 100;
        let t: Vec<f64> = linspace(0., 10., n).collect();
        let y: Vec<f64> = t.iter().map(|x| x.sin()).collect();
    
        let trace = Scatter::new(t, y).mode(Mode::Markers);
        let mut plot = Plot::new();
        plot.add_trace(trace);
        if show {
            plot.show();
        }
        if saveimg {
            plot.to_html("images/scatter_plot.html");
        }
    }

    simple_scatter_plot(false, true);

    Ok(())
}