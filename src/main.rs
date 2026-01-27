//https://crates.io/crates/yahoo_finance_api
use charton::prelude::*;
use chrono::NaiveDate;
use plotly::common::Title;
use plotly::{Plot, Scatter};
use polars::prelude::*;
use std::error::Error;

use lib::*;
mod lib;
fn main() {
    let closing_prices = get_price_history("NVDA".to_string());

    //calculate 20-day simple moving averages of closing prices.
    let moving_averages: Vec<f64> = calculate_simple_moving_average(closing_prices.clone(), 20);

    //calculate the standard deviation of the moving averages
    let standard_deviations = calculate_sma_std(closing_prices.clone(), 20);

    //calculate the bollinger bands
    let upper_band: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(a, b)| a + (2.0 * b))
        .collect();

    let lower_band: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(a, b)| a - (2.0 * b))
        .collect();

    let x_values: Vec<NaiveDate> = get_last_twenty_days();
    let last_two: Vec<NaiveDate> = (*(&x_values[x_values.len() - 2..])).to_vec();

    let df = df![
        "dates" => last_two,
        "last_two_closing_prices" => (*(&closing_prices[closing_prices.len() - 2..])).to_vec(),
        "lower_band" =>lower_band,
       "upper_band" => upper_band
    ];

    println!("{:?}", df);

    let trace = Scatter::new(x_values, closing_prices).name("Line Chart");
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(plotly::Layout::new().title(Title::from("My Line Chart")));
    plot.show();
}
