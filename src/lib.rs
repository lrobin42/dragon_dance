use chrono::{DateTime, Local, NaiveDate, TimeDelta};
use plotly::Candlestick;
use plotly::Layout;
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::layout::LayoutGrid;
use plotly::{Plot, Scatter};
use std::collections::VecDeque;
use ta::Next;
use ta::indicators::MovingAverageConvergenceDivergence as Macd;
use ta::indicators::RelativeStrengthIndex;
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
pub fn _get_price_history(ticker: &String) -> Vec<f64> {
    let provider = yahoo::YahooConnector::new().unwrap();
    let response = tokio_test::block_on(provider.get_quote_range(&ticker, "1d", "2y")).unwrap(); //2y
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

pub fn calculate_simple_moving_average(price_array: &Vec<f64>, window: i32) -> Vec<f64> {
    let interval = window as usize;
    let length = price_array.len();
    let mut sma_array = Vec::new();

    // Start when we have enough data points
    for index in interval..=length {
        let start_index = index - interval;
        let interval_slice = &price_array[start_index..index];
        let sum: f64 = interval_slice.iter().sum();
        sma_array.push(sum / interval as f64);
    }

    sma_array
}

// create a function to calculate the standard deviation for every 20-day period
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

pub fn _get_last_twenty_days() -> Vec<NaiveDate> {
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

pub fn _last_twenty_entries<T: Clone>(vector: Vec<T>) -> Vec<T> {
    let start = vector.len().saturating_sub(20);
    vector[start..].to_vec()
}

#[derive(Debug, Clone)]
pub struct PriceHistory {
    pub dates: Vec<NaiveDate>,
    pub open: Vec<f64>,
    pub high: Vec<f64>,
    pub low: Vec<f64>,
    pub close: Vec<f64>,
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

pub fn calculate_macd(ticker: &String) -> [Vec<f64>; 3] {
    //let ticker: String = "COST".to_string();
    let candlestick_price_history = candlestick_price_history(ticker);
    let mut macd = Macd::new(12, 26, 9).unwrap();

    // Iterate through your price history
    let mut macd_series = Vec::new();
    let mut signal_series = Vec::new();
    let mut histogram_series = Vec::new();

    for candle in candlestick_price_history.close {
        // Use the closing price from each candlestick
        let result = macd.next(candle);
        macd_series.push(result.macd);
        signal_series.push(result.signal);
        histogram_series.push(result.histogram);
    }
    return [macd_series, signal_series, histogram_series];
}

//Plot bollinger bands, macd, rsi, and price history for stock/etf of choice
pub fn plot_indicators(ticker: String) {
    let candlestick_price_history = candlestick_price_history(&ticker);
    let cd_history = candlestick_price_history.clone();
    let closing_prices = candlestick_price_history.close.clone();

    //calculate 20-day simple moving averages of closing prices.
    let moving_averages: Vec<f64> = calculate_simple_moving_average(&closing_prices.clone(), 20);

    //calculate the standard deviation of the moving averages
    let standard_deviations = calculate_sma_std(&closing_prices, 20);

    //calculate the bollinger bands
    let upper_band: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(avg, std)| avg + (2.0 * std))
        .collect();

    let lower_band: Vec<f64> = moving_averages
        .iter()
        .zip(standard_deviations.iter())
        .map(|(avg, std)| avg - (2.0 * std))
        .collect();

    //calculate the macd so we can add that to the graph
    let [macd, signal, _histogram] = calculate_macd(&ticker);

    //calculate rsi
    let mut rsi = RelativeStrengthIndex::new(14).unwrap();
    let mut rsi_values = Vec::new();

    for price in closing_prices {
        let rsi_value = rsi.next(price);
        rsi_values.push(rsi_value);
    }

    let x_values = &candlestick_price_history.dates[19..];

    let upper_trace = Scatter::new(x_values.to_vec(), upper_band.clone()).name("Upper BB");

    let lower_trace = Scatter::new(x_values.to_vec(), lower_band.clone()).name("Lower BB");

    let title = format!("{ticker} indicators");
    let mut plot = Plot::new();

    // Price plot traces (assigned to first subplot)
    let upper_trace_subplot = upper_trace.x_axis("x").y_axis("y");
    let lower_trace_subplot = lower_trace.x_axis("x").y_axis("y");

    let candlestick_trace = Candlestick::new(
        x_values.to_vec(),
        cd_history.open[19..].to_vec(),
        cd_history.high[19..].to_vec(),
        cd_history.low[19..].to_vec(),
        cd_history.close[19..].to_vec(),
    )
    .name(ticker.clone())
    .show_legend(true)
    .x_axis("x")
    .y_axis("y");

    // MACD plot traces (assigned to second subplot)
    let macd_trace = Scatter::new((0..macd.len()).collect(), macd)
        .name("MACD")
        .x_axis("x2")
        .y_axis("y2");

    let signal_trace = Scatter::new((0..signal.len()).collect(), signal)
        .name("Signal")
        .x_axis("x2")
        .y_axis("y2");

    let rsi_trace = Scatter::new((0..rsi_values.len()).collect(), rsi_values)
        .name("RSI")
        .x_axis("x3")
        .y_axis("y3");

    // Add all traces
    plot.add_trace(upper_trace_subplot);
    plot.add_trace(lower_trace_subplot);
    plot.add_trace(Box::new(candlestick_trace));
    plot.add_trace(macd_trace);
    plot.add_trace(signal_trace);
    plot.add_trace(rsi_trace);

    // Set layout with 2 rows, 1 column
    plot.set_layout(
        Layout::new()
            .title(Title::from(title))
            .grid(
                LayoutGrid::new()
                    .rows(3)
                    .columns(1)
                    .pattern(plotly::layout::GridPattern::Independent),
            )
            .x_axis(Axis::new().title("").domain(&[0.0, 1.0]))
            .y_axis(Axis::new().title("Price").domain(&[0.68, 1.0])) // Top 68%
            .x_axis2(Axis::new().title("").domain(&[0.0, 1.0]))
            .y_axis2(Axis::new().title("MACD").domain(&[0.15, 0.4]))
            .x_axis3(Axis::new().title("").domain(&[0.0, 1.0]))
            .y_axis3(Axis::new().title("RSI").domain(&[0.0, 0.1])), //Bottom 10% of figure
    );

    plot.show();
}
