use reqwest::{ header::{ HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT }, Client };
use serde::{Deserialize, Serialize};
use std::env;
use crate::Result;

const API_KEY_ENV_PARAM: &str = "COINGECKO_API_KEY";

#[derive(Deserialize, Debug)]
pub struct CoinsMarketResponse {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String,
}

#[derive(Deserialize, Debug)]
pub struct CoinMarket {
    pub id: String,
    pub symbol: String,
    pub name: String,
    pub current_price: f64,
    pub ath: f64,
    pub ath_change_percentage: f64,
    pub ath_date: String,
    pub atl: f64,
    pub atl_change_percentage: f64,
    pub atl_date: String,
    pub market_cap: f64,
    pub price_change_percentage_24h: f64,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Coin {
    pub id: String,
    pub symbol: String,
    pub name: String,
}

#[allow(unused)]
pub struct CoinGeckoClient {}

#[allow(unused)]
impl CoinGeckoClient {
    pub fn new() -> Self {
        Self {}
    }

    #[allow(unused)]
    pub async fn get_coins_markets(
        &self,
        token_ids: Vec<String>,
        currency: &str
    ) -> Result<Vec<CoinMarket>> {
        let _currency = currency.to_lowercase();

        let base_url = "https://api.coingecko.com/api/v3/coins/markets";
        let ids = token_ids.join(","); // Join the token IDs with commas
        let currency = "usd"; // Target currency (you can make this dynamic if needed)

        // Build the full URL with query parameters
        //    let url = format!("{}?vs_currency={}&ids={}", base_url, currency, ids);
        let url = format!("{}", base_url);

        let api_key = env
            ::var(API_KEY_ENV_PARAM)
            .expect(format!("{} not defined in env", API_KEY_ENV_PARAM).as_str());

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Chrome"));
        //headers.insert("x-cg-pro-api-key", HeaderValue::from_str(&api_key)?);

        let query_params_api_key = vec![("x-cg-pro-api-key", api_key)];
        let query_params_currency = vec![("vs_currency", currency)];
        let query_params_ids = vec![("ids", ids)];

        //println!("{} {}", url, api_key);

        let client = Client::new();

        let request = client
            .get(url)
            .headers(headers)
            .query(&query_params_currency)
            .query(&query_params_ids);

        println!("Request:{:?}", request);

        let response = request.send().await?;

        let raw_text = response.text().await?;
        println!("Raw response: {}", raw_text);

        let result: Vec<CoinMarket> = serde_json::from_str(&raw_text)?;

        Ok(result)

        // Make the GET request using reqwest
        //let response = reqwest::get(&url).await?;

        // Parse the JSON response into a vector of CoinMarket structs
        //let coin_markets: Vec<CoinMarket> = response.json().await?;

        //Ok(coin_markets)
    }

    pub async fn list_coins(&self) -> Result<Vec<Coin>> {
        let url = "https://api.coingecko.com/api/v3/coins/list";

        let api_key = env
            ::var(API_KEY_ENV_PARAM)
            .expect(format!("{} not defined in env", API_KEY_ENV_PARAM).as_str());

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Chrome"));
        headers.insert("x-cg-pro-api-key", HeaderValue::from_str(&api_key)?);

        let client = Client::new();

        let request = client.get(url).headers(headers);

        //println!("Request:{:?}", request);

        let response = request.send().await?;

        let raw_text = response.text().await?;
        println!("Raw response: {}", raw_text);

        // Make the GET request using reqwest
        let response = reqwest::get(String::from(url)).await?;

        // Parse the JSON response into a vector of CoinsMarketResponse structs
        let coins: Vec<Coin> = response.json().await?;

        Ok(coins)
    }


    pub async fn get_coin(
        &self,
        token_id: &str,
        currency: &str
    ) -> Result<Vec<CoinMarket>> {
        let _currency = currency.to_lowercase();

        let base_url = "https://api.coingecko.com/api/v3/coins";
        let currency = "usd"; // Target currency (you can make this dynamic if needed)

        // Build the full URL with query parameters
        //    let url = format!("{}?vs_currency={}&ids={}", base_url, currency, ids);
        let url = format!("{}/{}", base_url, token_id);

        let api_key = env
            ::var(API_KEY_ENV_PARAM)
            .expect(format!("{} not defined in env", API_KEY_ENV_PARAM).as_str());

        let mut headers = HeaderMap::new();
        //headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        //headers.insert(USER_AGENT, HeaderValue::from_static("Chrome"));
        //headers.insert("x-cg-pro-api-key", HeaderValue::from_str(&api_key)?);

        let query_params_currency = vec![("vs_currency", currency)];
        let query_params_api_key = vec![("x-cg-pro-api-key", api_key)];
        //let query_params_id = vec![("id", token_id)];

        //println!("{} {}", url, api_key);

        let client = Client::new();

        let request = client
            .get(url)
            .headers(headers)
            .query(&query_params_api_key)
            .query(&query_params_currency);


        println!("Request:{:?}", request);

        let response = request.send().await?;

        let raw_text = response.text().await?;
        println!("Raw response: {}", raw_text);

        let result: Vec<CoinMarket> = serde_json::from_str(&raw_text)?;

        Ok(result)

        // Make the GET request using reqwest
        //let response = reqwest::get(&url).await?;

        // Parse the JSON response into a vector of CoinMarket structs
        //let coin_markets: Vec<CoinMarket> = response.json().await?;

        //Ok(coin_markets)
    }

}
