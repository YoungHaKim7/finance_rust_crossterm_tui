use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
pub const API_KEY: &str = include_str!("..\\key.txt");

#[cfg(target_os = "macos")]
pub const API_KEY: &str = include_str!(r"../../key.txt");

#[cfg(target_os = "linux")]
pub const API_KEY: &str = include_str!(r"../../key.txt");

struct FinanceClient {
    url: String,
    client: Client,
    search_string: String,
}

/// Serialize = into JSON
///
/// Deserialize = into Rust type
#[derive(Debug, Serialize, Deserialize)]
struct CompanyInfo {
    country: String,
    currency: String,
    exchange: String,
    #[serde(rename = "finnhubIndustry")]
    industry: String,
    ipo: String, // chrono -> NaiveDate
    #[serde(rename = "marketCapitalization")]
    market_capitalization: f64,
    name: String,
    phone: String,
    #[serde(rename = "shareOutstanding")]
    shares_outstanding: f64,
    ticker: String,
    url: String,
}

// {
//   "country": "US",
//   "currency": "USD",
//   "exchange": "NASDAQ/NMS (GLOBAL MARKET)",
//   "ipo": "1980-12-12",
//   "marketCapitalization": 1415993,
//   "name": "Apple Inc",
//   "phone": "14089961010",
//   "shareOutstanding": 4375.47998046875,
//   "ticker": "AAPL",
//   "weburl": "https://www.apple.com/",
//   "logo": "https://static.finnhub.io/logo/87cb30d8-80df-11ea-8951-00000000092a.png",
//   "finnhubIndustry":"Technology"
// }

impl FinanceClient {
    fn get_profile_by_symbol(&self) {
        let text = self
            .client
            .get(format!(
                // "{}/stock/finacials-reported?symbol={}",
                "{}/stock/profile2?symbol={}",
                self.url, self.search_string
            ))
            .header("X-Finnhub-Token", API_KEY)
            .send()
            .unwrap()
            .text()
            .unwrap();
        let company_info: CompanyInfo = serde_json::from_str(&text).unwrap();
        println!("Text: {text}");
    }
}

fn main() -> io::Result<()> {
    let mut client = FinanceClient {
        url: "https://finanhub.io/api/v1".to_string(),
        client: Client::default(),
        search_string: String::new(),
    };

    loop {
        // `read()` blocks until an `Event` is available
        match read()? {
            Event::Key(key_event) => {
                let KeyEvent {
                    code,
                    modifiers,
                    kind: _,
                    state: _,
                } = key_event;
                match (code, modifiers) {
                    (KeyCode::Char(c), _) => {
                        client.search_string.push(c);
                        println!("{}", client.search_string);
                    }
                    (KeyCode::Esc, _) => {
                        client.search_string.clear();
                        println!("{}", client.search_string);
                    }

                    (KeyCode::Backspace, _) => {
                        client.search_string.pop();
                        println!("{}", client.search_string);
                    }
                    (KeyCode::Enter, _) => {
                        client.get_profile_by_symbol();
                    }
                    (KeyCode::Up, _) => {
                        println!("Pressed up");
                    }
                    (KeyCode::Down, _) => {
                        println!("Pressed Down");
                    }
                    (KeyCode::Left, _) => {
                        println!("Pressed Left");
                    }
                    (KeyCode::Right, _) => {
                        println!("Pressed Right");
                    }
                    (_, _) => println!("error"),
                }
            }
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    // loop {
    //     // `read()` blocks until an `Event` is available
    //     match read()? {
    //         Event::FocusGained => println!("FocusGained"),
    //         Event::FocusLost => println!("FocusLost"),
    //         Event::Key(event) => println!("{:?}", event),
    //         Event::Mouse(event) => println!("{:?}", event),
    //         // #[cfg(feature = "bracketed-paste")]
    //         Event::Paste(data) => println!("{:?}", data),
    //         Event::Resize(width, height) => println!("New size {}x{}", width, height),
    //     }
    // }

    Ok(())
}

// tui

// FINANCE TOOL
// COMPANY DATA || Market cap || This week's news
// STOCK DATA || One stock data || Weekly data
