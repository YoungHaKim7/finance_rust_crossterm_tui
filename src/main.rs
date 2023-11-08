use std::io;

use crossterm::event::{read, KeyEvent, KeyCode, Event};

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
    let client = FinanceClient {
        url: "https://finanhub.io/api/v1".to_string(),
        client: Client::default(),
        search_string: String::new(),
    };
    loop {
        match read()? {
            Event::Key(key_event) => {
                let KeyEvent { code, modifiers } = key_event;
                match (code, modifiers) {
                    (KeyCode::Char(c), _) => {
                        client.search_string.push(c);
                        println!("{}", client.search_string);
                    },
                    (_, _) => {}
                }
            }
 
            Event::FocusGained => todo!(),
            Event::FocusLost => todo!(),
            Event::Mouse(_) => todo!(),
            Event::Paste(_) => todo!(),
            Event::Resize(_, _) => todo!(),       }
    }
    Ok(())
}
