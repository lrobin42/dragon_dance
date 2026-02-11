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
    plot_indicators("QQQ".to_string());
}
