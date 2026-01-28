use chrono::{DateTime, NaiveDate, Utc};
use plotly::Candlestick;
use plotly::Plot;
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let full_history = candlestick_price_history("NVDA".to_string());

    let trace = Candlestick::new(
        full_history.dates,
        full_history.open,
        full_history.high,
        full_history.low,
        full_history.close,
    )
    .name("NVDA")
    .show_legend(true);

    //Display or plot the trace
    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.show();
}

#[derive(Debug, Clone)]
pub struct PriceHistory {
    pub dates: Vec<NaiveDate>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
}

pub fn candlestick_price_history(ticker: String) -> PriceHistory {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(&ticker, "1d", "20d")).unwrap();
    let quotes = response.quotes().unwrap();

    let mut history = PriceHistory {
        dates: Vec::with_capacity(quotes.len()),
        open: Vec::with_capacity(quotes.len()),
        high: Vec::with_capacity(quotes.len()),
        low: Vec::with_capacity(quotes.len()),
        close: Vec::with_capacity(quotes.len()),
    };

    for quote in quotes {
        // Extract timestamp from quote and convert to NaiveDate
        let datetime =
            DateTime::from_timestamp(quote.timestamp as i64, 0).expect("Invalid timestamp");
        history.dates.push(datetime.date_naive());
        history.open.push(quote.open);
        history.high.push(quote.high);
        history.low.push(quote.low);
        history.close.push(quote.close);
    }

    history
}
