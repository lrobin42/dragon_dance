use dragon_dance::*;
use plotly::Candlestick;
use plotly::Layout;
use plotly::common::Title;
use plotly::layout::Axis;
use plotly::layout::LayoutGrid;
use plotly::{Plot, Scatter};
use ta::Next;
use ta::indicators::RelativeStrengthIndex;

fn main() {
    let ticker: String = "COST".to_string();
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

    let title = format!("{ticker} closing prices");
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
            .y_axis(Axis::new().title("Price").domain(&[0.68, 1.0])) // Top 70%
            .x_axis2(Axis::new().title("").domain(&[0.0, 1.0]))
            .y_axis2(Axis::new().title("MACD").domain(&[0.15, 0.4])) // Middle 15%
            .x_axis3(Axis::new().title("").domain(&[0.0, 1.0]))
            .y_axis3(Axis::new().title("RSI").domain(&[0.0, 0.1])),
    );

    plot.show();
}
