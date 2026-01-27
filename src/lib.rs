use chrono::prelude::*;
use chrono::{DateTime, Local, NaiveDate, TimeDelta, Utc};
use statistical::standard_deviation;
use std::collections::VecDeque;

use tokio_test;
use yahoo_finance_api as yahoo;

//Create a function to get the latest price on the security
pub fn _get_latest_price(_ticker: String) -> yahoo_finance_api::Quote {
    let provider = yahoo::YahooConnector::new().unwrap();

    // get the latest quotes in 1 day intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("NVDA", "1d")).unwrap();

    // extract just the latest valid quote summary including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();
    println!("The latest NVDA price is ${}", quote.close);
    quote
}

//Create a function to call the last 2 years of prices
pub fn get_price_history(ticker: &String) -> Vec<f64> {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(&ticker, "1d", "40d")).unwrap(); //2y
    let quotes = response.quotes().unwrap();

    let mut opening_prices = Vec::new();
    let mut closing_prices = Vec::new();
    let mut high_prices = Vec::new();
    let mut low_prices = Vec::new();

    //segment the price data
    for entry in &quotes {
        opening_prices.push(entry.open);
        closing_prices.push(entry.close);
        high_prices.push(entry.high);
        low_prices.push(entry.low);
    }
    closing_prices
}

pub fn calculate_simple_moving_average(price_array: Vec<f64>, window: i32) -> Vec<f64> {
    let interval = window as usize;
    let mut index = interval - 1;
    let length = price_array.len() + 1;
    let mut sma_array = Vec::new();

    while index < length {
        index += 1;

        let start_index = index - interval;
        let interval_slice = &price_array[start_index..index - 1];
        let sum: f64 = interval_slice.iter().sum();
        let interval_float = interval as f64;
        sma_array.push(sum / interval_float);
    }
    sma_array
}

//create a function to calculate the standard deviation for every 20-day period
pub fn calculate_sma_std(prices: &Vec<f64>, window: usize) -> Vec<f64> {
    let mut std_devs = Vec::new();
    let mut buffer: VecDeque<f64> = VecDeque::new();

    for &price in prices {
        buffer.push_back(price);
        if buffer.len() > window {
            buffer.pop_front();
        }

        if buffer.len() == window {
            let mean: f64 = buffer.iter().sum::<f64>() / window as f64;
            let variance: f64 =
                buffer.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / window as f64;
            let std_dev = variance.sqrt();
            std_devs.push(std_dev);
        }
    }

    std_devs
}

pub fn get_last_twenty_days() -> Vec<NaiveDate> {
    let today = Local::now().date_naive();
    let mut difference = 19;
    let mut dates = Vec::new();
    while difference > 0 {
        let x_days_ago = today.checked_sub_signed(TimeDelta::try_days(difference).unwrap());
        dates.push(x_days_ago.expect("REASON"));
        difference -= 1;
    }
    dates.push(today);
    dates
}

pub fn last_twenty_entries<T: Clone>(vector: Vec<T>) -> Vec<T> {
    let start = vector.len().saturating_sub(20);
    vector[start..].to_vec()
}
