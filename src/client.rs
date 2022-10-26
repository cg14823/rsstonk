use reqwest;
use serde::Deserialize;
use std::{env, error::Error, fmt, time::Duration};

const FINNHUB_ENDPOINT: &str = "https://finnhub.io/api/v1";
pub type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct APIError {
    details: String,
}

impl Error for APIError {
    fn description(&self) -> &str {
        &self.details
    }
}

impl fmt::Display for APIError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "API Error: {}", self.details)
    }
}

pub struct Client {
    client: reqwest::blocking::Client,
}

#[derive(Deserialize, Debug)]
pub struct SymbolQuote {
    // Current
    pub c: f64,
    // High
    pub h: f64,
    // Low
    pub l: f64,
    // Delta (change)
    pub d: f64,
    // Delta percent change
    pub dp: f64,
    // Open price
    pub o: f64,
    // Previous
    pub pc: f64,
    // Timestamp
    pub t: i64,
}

impl Client {
    pub fn get_symbol_price(&self, symbol: String) -> Result<SymbolQuote> {
        let endpoint = format!("{}/quote?symbol={}", FINNHUB_ENDPOINT, symbol);
        let res = match self.client.get(endpoint).send() {
            Ok(response) => response,
            Err(e) => return Err(Box::new(e)),
        };

        if !res.status().is_success() {
            return Err(Box::new(APIError {
                details: format!("unexpected status code {}", res.status().as_str()),
            }));
        }

        let quote = match res.json::<SymbolQuote>() {
            Ok(quote) => quote,
            Err(e) => return Err(Box::new(e)),
        };

        Ok(quote)
    }
}

pub fn get_finhub_client() -> Client {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "X-Finnhub-Token",
        reqwest::header::HeaderValue::from_str(
            env::var("FINNHUB_API_KEY")
                .expect("FINNHUB_API_KEY is not set")
                .as_str(),
        )
        .unwrap(),
    );

    return Client {
        client: reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .user_agent("rsstonk")
            .default_headers(headers)
            .build().expect("could not build client"),
    };
}
