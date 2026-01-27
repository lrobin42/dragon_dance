use chrono::NaiveDate;
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::{Plot, Scatter};
use polars::prelude::*;

use lib::*;
mod lib;

fn main() {
    let ticker: String = "NVDA".to_string();
    let closing_prices = get_price_history(&ticker);

    //calculate 20-day simple moving averages of closing prices.
    let moving_averages: Vec<f64> = calculate_simple_moving_average(closing_prices.clone(), 20);

    //calculate the standard deviation of the moving averages
    let standard_deviations = calculate_sma_std(&closing_prices.clone(), 20);

    //calculate the bollinger bands
    let upper_band: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(avg, std)| avg + (2.0 * std))
        .collect();

    let lower_band: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(a, b)| a - (2.0 * b))
        .collect();

    let x_values: Vec<NaiveDate> = get_last_twenty_days();

    let df = df![
        "dates" => &x_values,
        "lower_band" =>last_twenty_entries(lower_band.clone()),
        "upper_band" => last_twenty_entries(upper_band.clone())
    ];

    println!("{:?}", df);

    let price_trace = Scatter::new(x_values.clone(), closing_prices.clone()).name("Closing Price");
    let upper_trace =
        Scatter::new(x_values.clone(), upper_band.clone()).name("Upper Bollinger Band");
    let lower_trace =
        Scatter::new(x_values.clone(), lower_band.clone()).name("Lower Bollinger Band");

    let title = format!("{ticker} closing prices");
    let mut plot = Plot::new();

    plot.add_trace(upper_trace);
    plot.add_trace(price_trace);
    plot.add_trace(lower_trace);

    plot.set_layout(
        plotly::Layout::new()
            .title(Title::from(title))
            .x_axis(Axis::new().title("Date"))
            .y_axis(Axis::new().title("Price")),
    );

    plot.show();
}
