//https://crates.io/crates/yahoo_finance_api
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let quote = get_price_history("NVDA".to_string());

    for entry in &quote {
        println!("{:?}", entry.open)
    }
    //println!("{:?}", quote)
}
//Create a function to get the latest price on the security
fn get_latest_price(ticker: String) -> yahoo_finance_api::Quote {
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

//Create a function to calculate bollinger bands
