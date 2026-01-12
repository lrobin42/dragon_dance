//https://crates.io/crates/yahoo_finance_api
use statistical::standard_deviation;
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let quote = get_price_history("NVDA".to_string());

    let mut opening_prices = Vec::new();
    let mut closing_prices = Vec::new();
    let mut high_prices = Vec::new();
    let mut low_prices = Vec::new();

    //segment the price data
    for entry in &quote {
        opening_prices.push(entry.open);
        closing_prices.push(entry.close);
        high_prices.push(entry.high);
        low_prices.push(entry.low);
    }
    //calculate 20-day simple moving averages of closing prices.
    let mut moving_averages: Vec<f64> = calculate_simple_moving_average(closing_prices.clone(), 20);

    //calculate the standard deviation of the moving averages
    let standard_deviations = calculate_sma_std(closing_prices.clone(), 20);

    let upper_bandt: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(a, b)| a + (2.0 * b))
        .collect();
}
//Create a function to get the latest price on the security
fn _get_latest_price(_ticker: String) -> yahoo_finance_api::Quote {
    let provider = yahoo::YahooConnector::new().unwrap();
    // get the latest quotes in 1 minute intervals
    let response = tokio_test::block_on(provider.get_latest_quotes("NVDA", "1d")).unwrap();
    // extract just the latest valid quote summery
    // including timestamp,open,close,high,low,volume
    let quote = response.last_quote().unwrap();
    println!("The latest NVDA price is ${}", quote.close);
    quote
}
//let time: OffsetDateTime = OffsetDateTime::from_unix_timestamp(quote.timestamp).unwrap();

//Create a function to call the last 2 years of prices
fn get_price_history(ticker: String) -> Vec<yahoo_finance_api::Quote> {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(&ticker, "1d", "2y")).unwrap();
    let quotes = response.quotes().unwrap();
    quotes
}

fn calculate_simple_moving_average(price_array: Vec<f64>, window: i32) -> Vec<f64> {
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
fn calculate_sma_std(price_array: Vec<f64>, window: i32) -> Vec<f64> {
    let interval = window as usize;
    let mut index = interval - 1;
    let length = price_array.len() + 1;
    let mut std_array = Vec::new();

    while index < length {
        index += 1;

        let start_index = index - interval;
        let interval_slice = &price_array[start_index..index - 1];
        let std_dev = statistical::standard_deviation(interval_slice, None);
        std_array.push(std_dev);
    }
    std_array
}

//Create a function to calculate bollinger bands
