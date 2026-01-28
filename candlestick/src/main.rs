use chrono::{DateTime, Local, NaiveDate, TimeDelta, Utc};
use plotly::Candlestick;
use tokio_test;
use yahoo_finance_api as yahoo;

fn main() {
    let dates = get_last_twenty_days();
    let full_history = candlestick_price_history("NVDA".to_string());
    //println!("{:?}", full_history)

    let open = full_history.open;
    let high = full_history.high;
    let low = full_history.low;
    let close = full_history.close;

    let trace = Candlestick::new(dates, open, high, low, close)
        .name("candle_test")
        .show_legend(true);

    let expected = serde_json::json!({
        "type": "candlestick",
        "x": dates,
        "open": [5, 6],
        "high": [9, 10],
        "low": [3, 5],
        "close": [6, 9],
        "increasing": {"line": {"color": "green", "width": 1.0}},
        "decreasing": {"line": {"color": "red", "width": 1.0}},
        "name": "trace_1",
        "showlegend": true
    });
}
#[derive(Debug, Clone)]
pub struct PriceHistory {
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
        open: Vec::with_capacity(quotes.len()),
        high: Vec::with_capacity(quotes.len()),
        low: Vec::with_capacity(quotes.len()),
        close: Vec::with_capacity(quotes.len()),
    };

    for quote in quotes {
        history.open.push(quote.open);
        history.high.push(quote.high);
        history.low.push(quote.low);
        history.close.push(quote.close);
    }

    history
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
