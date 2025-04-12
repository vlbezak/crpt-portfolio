use reqwest::{ header::{ HeaderMap, HeaderValue, CONTENT_TYPE, USER_AGENT }, Client };
use serde::Deserialize;
use std::{ collections::HashMap, env };
use crate::Result;


const API_KEY_ENV_PARAM: &str = "CRYPTOCOMPARE_API_KEY";

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct PriceMultiFullResponse {
    pub raw: HashMap<String, HashMap<String, Level2>>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "UPPERCASE")]
pub struct Level2 {
    #[serde(rename = "FROMSYMBOL")]
    pub from_symbol: String,
    pub price: f64,
    #[serde(rename = "MKTCAP")]
    pub mktcap: f64,
    #[serde(rename = "CIRCULATINGSUPPLYMKTCAP")]
    pub circulating_supply_mktcap: f64,
    #[serde(rename = "CHANGEPCT24HOUR")]
    pub change_pct_24_hour: f64,

}

pub struct CryptoCompareClient {}

impl CryptoCompareClient {
    pub fn new() -> Self {
        Self {}
    }

    pub async fn get_coin_info(
        &self,
        token_ids: &Vec<&str>,
        currencies: &Vec<&str>
    ) -> Result<PriceMultiFullResponse> {
        let base_url = "https://min-api.cryptocompare.com/data/pricemultifull";
        let ids = token_ids.join(","); // Join the token IDs with commas
        let currencies = currencies.join(","); // Join currencies with comas

        let api_key = env
            ::var(API_KEY_ENV_PARAM)
            .expect(format!("{} not defined in env", API_KEY_ENV_PARAM).as_str());

        let mut headers = HeaderMap::new();
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        headers.insert(USER_AGENT, HeaderValue::from_static("Chrome"));
        headers.insert(
            "Authorization",
            HeaderValue::from_str(format!("ApiKey {}", &api_key).as_str())?
        );

        let query_params_currency = vec![("tsyms", currencies)];
        let query_params_ids = vec![("fsyms", ids)];

        //println!("{}", base_url);

        let client = Client::new();

        let request = client
            .get(base_url)
            .headers(headers)
            .query(&query_params_currency)
            .query(&query_params_ids);

        //println!("Request:{:?}", request);

        let response = request.send().await?;

        let raw_text = response.text().await?;
        //println!("Raw response: {}", raw_text);
        
        let result: PriceMultiFullResponse = serde_json::from_str(&raw_text)?;

        //println!("{:#?}", result);

        Ok(result)
    }
}
