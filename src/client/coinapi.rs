
use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client};
use serde::Deserialize;
use std::env;
use crate::Result;
use crate::model::PriceInfo;
use crate::model::Currency;

use super::CurrentPriceApi;

const API_KEY_ENV_PARAM: &str  = "COINAPI_API_KEY";

#[derive(Deserialize, Debug)]
pub struct ExchangeRateResponse {
    pub asset_id_base: String,
    pub rates: Vec<Rate>,
}

#[derive(Deserialize, Debug)]
pub struct Rate {
    pub time: String,
    pub asset_id_quote: String,
    pub rate: f64,
}

pub struct CoinAPIClient {

}

impl CoinAPIClient {
    pub fn new() -> Self {
        Self {}
    }
}

impl CurrentPriceApi for CoinAPIClient {
    async fn get_prices(&self, symbol: &str) -> Result<Vec<PriceInfo>> {
        let response: ExchangeRateResponse = get_coin_price(symbol).await?;

        let rate_usd = response.rates
        .into_iter()
        .filter(|rate| rate.asset_id_quote == "USD")
        .next();

        let Some(val) = rate_usd else {
            return Err(format!("Cannot find USD rate for: {}", symbol).into());
        };
        
        let mut prices = Vec::new();
        prices.push(PriceInfo { 
            coin: String::from(symbol),
            currency: Currency::USD,
            value: val.rate,
        });

        Ok(prices)
    }
}

async fn get_coin_price(symbol :&str) -> Result<ExchangeRateResponse> {
    let url = format!("https://rest.coinapi.io/v1/exchangerate/{}", symbol);

    let api_key = env::var(API_KEY_ENV_PARAM).expect(format!("{} not defined in env", API_KEY_ENV_PARAM).as_str());

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
    headers.insert("X-CoinAPI-Key", HeaderValue::from_str(&api_key)?);

    let result_currencies = "USD";

    let query_params = vec![("filter_asset_id", result_currencies)];

    let client = Client::new();

    let response = client
        .get(url)
        .headers(headers)
        .query(&query_params)
        .send().await?
        .json::<ExchangeRateResponse>().await?;

    Ok(response)

}

