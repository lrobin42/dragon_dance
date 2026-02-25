use anyhow::Result;
use chrono::NaiveDateTime;
use chrono::{DateTime, NaiveDate, Utc};
use polars::df;
use polars::prelude::*;
use std::collections::HashMap;
use yfinance_rs::{Interval, Range, Ticker, YfClient};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    //-> Result<(), Box<dyn std::error::Error>> {
    let client = YfClient::default();
    let ticker = Ticker::new(&client, "COST");
    let expirations: Vec<i64> = ticker.options().await?;

    //     let nanos = 0;
    //     let expirations: Vec<NaiveDate> = expirations_unix
    //         .iter()
    //         .filter_map(|&date| {
    //             DateTime::from_timestamp(date as i64, nanos as u32).map(|dt| dt.date_naive())
    //         })
    //         .collect();

    //     println!("{:?}", expirations);
    //     Ok(())
    // }

    let mut call_options: HashMap<String, f64> = HashMap::new();

    let mut symbols = Vec::new();
    let mut strikes = Vec::new();
    let mut prices = Vec::new();
    let mut bids = Vec::new();
    let mut asks = Vec::new();
    let mut expirations = Vec::new();
    let mut iv_s = Vec::new();
    //let mut greeks = Vec::new();

    // Step 2: Fetch option chain for a specific expiration
    if let Some(nearest) = expirations.first() {
        let chain = ticker.option_chain(Some(*nearest)).await?;

        // Access calls and puts
        println!("{:?}", chain.calls[0]); //.len());

        for entry in chain.calls {
            symbols.push(entry.contract_symbol);
            strikes.push(entry.strike);
            prices.push(entry.last_price);
            bids.push(entry.bid);
            asks.push(entry.ask);
            expirations.push(entry.expiration);
            iv_s.push(entry.implied_volatility);
        }
    }

    //println!("Number of puts: {}", chain.puts.len());

    //Ok(())

    let df = df! [
        "symbols" => symbols,
        "strikes" => strikes,
        "prices" => prices
    ];

    println!("{:?}", df);
    Ok(())
}
