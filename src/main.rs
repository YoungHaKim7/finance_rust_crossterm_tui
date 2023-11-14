use std::io;

use crossterm::event::{read, Event, KeyCode, KeyEvent, KeyEventKind};

use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[cfg(target_os = "windows")]
pub const API_KEY: &str = include_str!("..\\..\\key.txt");

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
    ipo: String, // chrono -> NaiveDate
    #[serde(rename = "marketCapitalization")]
    market_capitalization: f64,
    name: String,
    phone: String,
    #[serde(rename = "shareOutstanding")]
    shares_outstanding: f64,
    ticker: String,
    weburl: String,
    logo: String,
    #[serde(rename = "finnhubIndustry")]
    industry: String,
}

impl std::fmt::Display for CompanyInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let CompanyInfo {
            country,
            currency,
            exchange,
            ipo,
            market_capitalization,
            name,
            phone,
            shares_outstanding,
            ticker,
            weburl,
            logo,
            industry,
        } = self;

        let company_info = format!(
            "
Country: {country},
Currency: {currency}
Exchange: {exchange}
Ipo: {ipo},
Market_capitalization: {market_capitalization}
Name: {name}
Phone: {phone}
Shares_outstanding: {shares_outstanding}
Ticker: {ticker}  
Weburl: {weburl} 
Logo: {logo}
Industry: {industry}
            "
        );
        write!(f, "{}", company_info)
    }
}

impl FinanceClient {
    fn get_profile_by_symbol(&self) {
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
        let company_info: CompanyInfo = serde_json::from_str(&text).unwrap();
        println!("Text: {}", text);
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
                    state,
                    kind,
                } = key_event;
                match (code, modifiers, state, kind) {
                    (KeyCode::Char(c), _, _, KeyEventKind::Press) => {
                        client.search_string.push(c);
                        println!("{}", client.search_string);
                    }
                    (KeyCode::Esc, _, _, KeyEventKind::Press) => {
                        client.search_string.clear();
                        println!("{}", client.search_string);
                    }

                    (KeyCode::Backspace, _, _, KeyEventKind::Press) => {
                        client.search_string.pop();
                        println!("{}", client.search_string);
                    }
                    (KeyCode::Enter, _, _, KeyEventKind::Press) => {
                        client.get_profile_by_symbol();
                    }
                    (KeyCode::Up, _, _, KeyEventKind::Press) => {
                        println!("Pressed up");
                    }
                    (KeyCode::Down, _, _, KeyEventKind::Press) => {
                        println!("Pressed Down");
                    }
                    (KeyCode::Left, _, _, KeyEventKind::Press) => {
                        println!("Pressed Left");
                    }
                    (KeyCode::Right, _, _, KeyEventKind::Press) => {
                        println!("Pressed Right");
                    }
                    (_, _, _, _) => println!("error"),
                }
            }
            Event::FocusGained => {}
            Event::FocusLost => {}
            Event::Mouse(_) => {}
            Event::Paste(_) => {}
            Event::Resize(_, _) => {}
        }
    }

    Ok(())
}
