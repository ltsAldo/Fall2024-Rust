use serde::Deserialize;
use std::error::Error;
use std::fs;
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Asset {
    name: &'static str,
    api_url: Option<&'static str>,
    file_path: &'static str,
}

impl Asset {
    fn fetch_price(&self) -> Result<f64, Box<dyn Error>> {
        if let Some(api_url) = self.api_url {
            let response = ureq::get(api_url).call()?;
            let json: serde_json::Value = serde_json::from_reader(response.into_reader())?;
            match self.name {
                "Bitcoin" => Ok(json["bpi"]["USD"]["rate_float"].as_f64().unwrap_or(0.0)),
                "Ethereum" => Ok(json["data"]["priceUsd"].as_str().unwrap_or("0.0").parse::<f64>()?),
                _ => Err("Unknown asset".into()),
            }
        } else {
            Ok(4500.0) // Mocked value for SP500
        }
    }

    fn save_to_file(&self, price: f64) -> Result<(), std::io::Error> {
        fs::write(self.file_path, format!("{} Price: ${}", self.name, price))
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let assets = vec![
        Asset {
            name: "Bitcoin",
            api_url: Some("https://api.coindesk.com/v1/bpi/currentprice/BTC.json"),
            file_path: "bitcoin_price.txt",
        },
        Asset {
            name: "Ethereum",
            api_url: Some("https://api.coincap.io/v2/assets/ethereum"),
            file_path: "ethereum_price.txt",
        },
        Asset {
            name: "SP500",
            api_url: None, // No API for S&P 500
            file_path: "sp500_price.txt",
        },
    ];

    loop {
        for asset in &assets {
            match asset.fetch_price() {
                Ok(price) => {
                    asset.save_to_file(price)?;
                    println!("Fetched and saved: {} Price: ${}", asset.name, price);
                }
                Err(e) => println!("Failed to fetch price for {}: {}", asset.name, e),
            }
        }
        thread::sleep(Duration::from_secs(10));
    }
}
