extern crate cfonts;
mod client;
use clap::Parser;
use chrono::{NaiveDateTime};


use cfonts::{ say, Options };

#[derive(Parser)] // requires `derive` feature
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(last = true)]
    symbol: String,
}

fn main() {
    let args = Cli::parse();

    let finnhub_client = client::get_finhub_client();
    let symbol_price = finnhub_client.get_symbol_price(args.symbol).unwrap();
    say(Options { text: format!("${}", symbol_price.c) , ..Options::default()});
    println!("{} high: ${} low: ${}", NaiveDateTime::from_timestamp(symbol_price.t, 0), symbol_price.h, symbol_price.l)
}
