#![feature(option_result_contains)]

use get_tickers::get_ticker_names;

mod get_tickers;
mod quote;

fn main() {
    get_ticker_names();
}
