use iso_currency::Currency;
use std::collections::HashMap;

use crate::{RawCurrencyRate, utils::CurrencyRate};

pub async fn fetch_currencies() -> Option<HashMap<String, CurrencyRate>> {
    let resp = reqwest::get("https://api.frankfurter.dev/v2/rates?base=usd").await;
    if resp.is_ok() {
        let raw: Vec<RawCurrencyRate> = serde_json::from_str(
            &resp
                .unwrap()
                .text()
                .await
                .expect("failed to parse currencies ?! (1)"),
        )
        .expect("failed to parse currencies ?! (2)");

        return Some(
            raw.iter()
                .map(|f| {
                    let data = Currency::from_code(&f.quote)
                        .unwrap_or(Currency::from_code("USD").expect("what the hell"));

                    (
                        String::from(&f.quote),
                        CurrencyRate {
                            short: String::from(&f.quote),

                            symbol: data.symbol().to_string(),
                            full: String::from(data.name()),

                            from_currency: String::from("USD"),
                            from_amt: 1.0,
                            to: f.rate,
                        },
                    )
                })
                .collect(),
        );
    }

    None
}
