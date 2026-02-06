use chrono::{DateTime, Local, NaiveDate, TimeDelta};
use ta::Next;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;
use yahoo_finance_api as yahoo;

fn main() {
    let ticker: String = "COST".to_string();
    let candlestick_price_history = candlestick_price_history(&ticker);
    let mut macd = Macd::new(12, 26, 9).unwrap();

    // Iterate through your price history
    for candle in candlestick_price_history.close {
        // Use the closing price from each candlestick
        let result = macd.next(candle);
        println!("{:?}", result);
    }
}

pub fn candlestick_price_history(ticker: &String) -> PriceHistory {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(&ticker, "1d", "2y")).unwrap();
    let quotes = response.quotes().unwrap();

    // Skip first 19 quotes to align with 20-day moving average calculations
    let quotes_slice = &quotes; //[19..];

    let mut history = PriceHistory {
        dates: Vec::with_capacity(quotes_slice.len()),
        open: Vec::with_capacity(quotes_slice.len()),
        high: Vec::with_capacity(quotes_slice.len()),
        low: Vec::with_capacity(quotes_slice.len()),
        close: Vec::with_capacity(quotes_slice.len()),
    };

    for quote in quotes_slice {
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

#[derive(Debug, Clone)]
pub struct PriceHistory {
    pub dates: Vec<NaiveDate>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
}
// Returns (macd_line, signal_line, histogram)
/*MACD (Moving Average Convergence Divergence) is calculated from price data using three main components:
The basic calculation:

MACD Line: Subtract the 26-period EMA (Exponential Moving Average) from the 12-period EMA

MACD Line = 12-period EMA - 26-period EMA


Signal Line: Calculate a 9-period EMA of the MACD Line itself

Signal Line = 9-period EMA of MACD Line


MACD Histogram: Subtract the Signal Line from the MACD Line

Histogram = MACD Line - Signal Line



How to calculate an EMA:
The EMA gives more weight to recent prices. For any period N:

Multiplier = 2 / (N + 1)
EMA today = (Price today × Multiplier) + (EMA yesterday × (1 - Multiplier))

For the first EMA value, you typically use a simple moving average as the starting point.
Example workflow:
If you're working with daily closing prices, you'd calculate the 12-day EMA and 26-day EMA of those closing prices, subtract them to get the MACD line, then calculate a 9-day EMA of that MACD line to get your signal line.
The standard periods (12, 26, 9) were originally designed for daily charts, but traders often adjust them for different timeframes or trading styles. */
