use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent};

use reqwest::blocking::Client;

#[cfg(target_os = "windows")]
pub const API_KEY: &str = include_str!("..\\..\\key.txt");

#[cfg(target_os = "macos")]
pub const API_KEY: &str = include_str!(r"../../key.txt");

struct FinanceClient {
    url: String,
    client: Client,
    search_string: String,
}

struct CompanyInfo {
    country: String,
    currency: String,
    // marketCapitalization
    market_capitalization: String,
}

impl FinanceClient {
    fn get_profile_by_isin(&self) {
        let text = self
            .client
            .get(format!(
                "{}/stock/profile2?symbol={}",
                self.url, self.search_string
            ))
            .header("X-Finnhub-Token", API_KEY)
            .send()
            .unwrap()
            .text()
            .unwrap();
        println!("Text: {text}");
    }
}

fn main() -> io::Result<()> {
    // let client = FinanceClient {
    //     url: "https://finanhub.io/api/v1".to_string(),
    //     client: Client::default(),
    //     search_string: String::new(),
    // };

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::FocusGained => println!("FocusGained"),
            Event::FocusLost => println!("FocusLost"),
            Event::Key(event) => println!("{:?}", event),
            Event::Mouse(event) => println!("{:?}", event),
            // #[cfg(feature = "bracketed-paste")]
            Event::Paste(data) => println!("{:?}", data),
            Event::Resize(width, height) => println!("New size {}x{}", width, height),
        }
    }
    Ok(())
}
