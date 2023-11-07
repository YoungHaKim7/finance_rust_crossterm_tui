use std::io;

use crossterm::event::{read, KeyEvent};

use reqwest::blocking::Client;

// WindowsOS
// pub const API_KEY: &str = include_str!("..\\key.txt");

// macOS
pub const API_KEY: &str = include_str!(r"../../key.txt");

struct FinanceClient {
    url: String,
    client: Client,
}

struct CompanyInfo {
    country: String,
    currency: String,
    // marketCapitalization
    market_capitalization: String,
}

impl FinanceClient {
    fn get_profile_by_isin(&self, isin: &str) {
        let text = self
            .client
            .get(format!("{}/stock/profile2?isin={isin}", self.url))
            .header("X-Finnhub-Token", API_KEY)
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("Text: {text}");
    }
}

fn main() -> io::Result<()> {
    let client = FinanceClient {
        url: "https://finanhub.io/api/v1".to_string(),
        client: Client::default(),
    };
    Ok(())
}
