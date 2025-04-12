use async_trait::async_trait;

use crate::{
    client::cryptocompare::{ CryptoCompareClient, PriceMultiFullResponse },
    config::coins::CoinDef,
    model::{ Currency, PriceInfo },
};
use crate::Result;

use std::str::FromStr;

use super::PriceProvider;

pub struct CryptoComparePriceProvider {}

const MAX_SYMBOLS_LEN: usize = 200;
const SYMBOLS_TOLLERANCE: usize = 10;

#[async_trait]
impl PriceProvider for CryptoComparePriceProvider {
    async fn get_price(
        &self,
        symbol: &str,
        _additional_data: &Option<std::collections::HashMap<String, String>>,
        currencies: &Vec<Currency>
    ) -> Result<Vec<PriceInfo>> {
        let client = CryptoCompareClient::new();
        let mut token_ids = Vec::new();
        token_ids.push(symbol);

        let currencies_str: Vec<&str> = convert_currencies(&currencies);
        let resp = client.get_coin_info(&token_ids, &currencies_str).await?;
        convert_response(&resp)
    }

    async fn get_prices(
        &self,
        coins_definitions: &Vec<CoinDef>,
        currencies: &Vec<Currency>
    ) -> Result<Vec<PriceInfo>> {

        println!("CryptoCompare: get_prices:");
        println!("Coins: {:?}", coins_definitions.len());
        println!("Currencies: {:?}", currencies);

        let mut result = Vec::new();

        let client = CryptoCompareClient::new();

        let currencies_str: Vec<&str> = convert_currencies(&currencies);

        let all_coins = split_coins_inputs(&coins_definitions);
        if all_coins.is_empty() {
            println!("No input coins found");
            return Ok(Vec::new());
        }

        for group in all_coins {
            println!("Making request for {:?}", group);
            match client.get_coin_info(&group, &currencies_str).await {
                Ok(res) => {
                    let mut res: Vec<PriceInfo> = convert_response(&res)?;
                    result.append(&mut res);
                }
                Err(err) => {
                    println!(
                        "Problem getting price data for: {:?}: {:?}",
                        group,
                        err
                    );
                }
            }
        }
        Ok(result)
    }
}

fn split_coins_inputs(coins_definitions: &Vec<CoinDef>) -> Vec<Vec<&str>> {

    let mut result = Vec::new();
    if coins_definitions.is_empty() {
        return result;
    }

    let mut length = 0;
    let mut current_group = Vec::new();
    
    for coin_def in coins_definitions {
        
        let len_with_coma = &coin_def.code.len() + 1;
        if length + len_with_coma >= MAX_SYMBOLS_LEN - SYMBOLS_TOLLERANCE {
            result.push(current_group);
            current_group = Vec::new();
            length = 0;
            
        }
        current_group.push(&coin_def.code);
        length += len_with_coma;
    }

    if !current_group.is_empty() {
        result.push(current_group);
    }

    result
}

fn convert_response(response: &PriceMultiFullResponse) -> Result<Vec<PriceInfo>> {
    let mut result_prices = Vec::new();

    for (key, val) in response.raw.iter() {
        let symbol = key;
        for (curr, level2) in val {
            let price_info = PriceInfo {
                coin: symbol.to_string(),
                currency: Currency::from_str(curr)?,
                value: level2.price,
                market_cap: level2.circulating_supply_mktcap,
                change_24h: level2.change_pct_24_hour,
            };
            result_prices.push(price_info);
        }
    }

    Ok(result_prices)
}

fn convert_currencies(currencies: &Vec<Currency>) -> Vec<&str> {
    let currencies_str: Vec<&str> = currencies
    .iter()
    .map(|cur| cur.to_uppercase_str())
    .collect();

    currencies_str
}
