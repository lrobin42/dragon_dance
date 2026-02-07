use dragon_dance::*;
use plotly::Candlestick;
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::{Plot, Scatter};

fn main() {
    let ticker: String = "COST".to_string();
    let candlestick_price_history = candlestick_price_history(&ticker);
    let cd_history = candlestick_price_history.clone();

    let closing_prices = candlestick_price_history.close.clone(); //get_price_history(&ticker);

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

    let x_values = &candlestick_price_history.dates[19..];

    let upper_trace =
        Scatter::new(x_values.to_vec(), upper_band.clone()).name("Upper Bollinger Band");

    let lower_trace =
        Scatter::new(x_values.to_vec(), lower_band.clone()).name("Lower Bollinger Band");

    let title = format!("{ticker} closing prices");

    let mut plot = Plot::new();
    plot.add_trace(upper_trace);
    plot.add_trace(lower_trace);
    plot.set_layout(
        plotly::Layout::new()
            .title(Title::from(title))
            .x_axis(Axis::new().title("Date"))
            .y_axis(Axis::new().title("Price")),
    );

    let trace = Candlestick::new(
        x_values.to_vec(),
        cd_history.open[19..].to_vec(),
        cd_history.high[19..].to_vec(),
        cd_history.low[19..].to_vec(),
        cd_history.close[19..].to_vec(),
    )
    .name(ticker.clone())
    .show_legend(true);
    plot.add_trace(Box::new(trace));
    plot.show();

    //macd_plot
    let macd_trace = Scatter::new((0..macd.len()).collect(), macd).name("MACD");
    let signal_trace = Scatter::new((0..signal.len()).collect(), signal).name("signal");
    //let histogram_trace = Scatter::new((0..histogram.len()).collect(), histogram).name("histogram");

    let mut plot = Plot::new();
    plot.add_trace(signal_trace);
    //plot.add_trace(histogram_trace);
    plot.set_layout(
        plotly::Layout::new().title(Title::from("MACD")), //  .x_axis(Axis::new().title("Date"))
                                                          //.y_axis(Axis::new().title("Price")),
    );

    plot.add_trace(macd_trace);
    plot.show()
}
