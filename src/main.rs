//https://crates.io/crates/yahoo_finance_api
use chrono::prelude::*;
use chrono::{DateTime, Local, TimeDelta, Utc};
use lib::*;
mod lib;
fn main() {
    let closing_prices = get_price_history("NVDA".to_string());

    //calculate 20-day simple moving averages of closing prices.
    let mut moving_averages: Vec<f64> = calculate_simple_moving_average(closing_prices.clone(), 20);

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
    let test = get_last_twenty_days();
    println!("{:?}", test)
}
pub fn get_last_twenty_days() -> Vec<NaiveDate> {
    let today = Local::now().date_naive();
    let mut difference = 20;
    let mut dates = Vec::new();
    while difference > 0 {
        let x_days_ago = today.checked_sub_signed(TimeDelta::try_days(difference).unwrap());
        dates.push(x_days_ago.expect("REASON"));
        difference -= 1;
    }
    dates.push(today);
    dates
}
