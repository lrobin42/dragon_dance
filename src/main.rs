//https://crates.io/crates/yahoo_finance_api
use charton::prelude::*;
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
    let x_values = get_last_twenty_days(); //x-axis for plots will be the last 20 days

    let df = df![
        "dates" => x_values,
        "closing_prices" => closing_prices,
        "lower_band" =>lower_band,
       "upper_band" => upper_band
    ]; //?;

    println!("{:?}", df) //.head(Some(5)))

    //     // Create a line chart layer
    //     let line = Chart::build(&df)?
    //         .mark_line() // Line chart
    //         .encode((
    //             x("dates"),      // Map length column to X-axis
    //             y("lower_band"), // Map width column to Y-axis
    //         ))?;

    //     // Create a scatter point layer
    //     let scatter = Chart::build(&df)?
    //         .mark_point() // Scatter plot
    //         .encode((
    //             x("dates"),          // Map length column to X-axis
    //             y("closing_prices"), // Map width column to Y-axis
    //         ))?;

    //     LayeredChart::new()
    //         .add_layer(line) // Add the line layer
    //         .add_layer(scatter) // Add the scatter point layer
    //         .save("./layeredchart.svg")?;

    //     Ok(())
    //
}
