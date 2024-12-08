use std::collections::HashMap;

use crate::coins::CoinPriceStore;
use crate::config::coins::{read_default_coins_config, CoinDef, CoinsData, PriceProviderEnum};
use crate::model::{Currency, PriceInfo};
use crate::provider::get_price_provider;
use crate::{provider, Result};
use crate::coins::filestore::CoinPriceFileStore;

pub async fn update_coins_prices(currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>> {
    let coin_price_store = CoinPriceFileStore {
        dir_name: String::from("data"),
    };
    
    println!("Getting coins config");

    let coins_data: CoinsData = read_default_coins_config()?;
    let price_info = get_coins_prices_for_coins_data(&coins_data, currencies).await?;

    println!("Storing prices: {}", price_info.len());
    coin_price_store.write_prices(&price_info)?;
    Ok(Vec::new())
}

async fn get_coins_prices_for_coins_data(coins_data: &CoinsData, currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>> {
    let mut result_prices: Vec<PriceInfo> = Vec::new();

    let mut providers: HashMap<&PriceProviderEnum, Vec<CoinDef>> = HashMap::new();
    for coin_def in coins_data.coins.iter() {
        providers.entry(&coin_def.price_provider)
        .or_insert_with(Vec::new)
        .push(coin_def.clone());
    }

    println!("Providers: {:#?}", providers);
    for (provider, coins) in providers {
        println!("Processing provider:{:#?}", provider);
        let provider_impl = get_price_provider(&provider);
        let mut prices = provider_impl.get_prices(&coins, &currencies).await?;
        result_prices.append(&mut prices);
    }

    println!("Result prices: {:#?}", result_prices);

    Ok(result_prices)
}




// async fn get_coins_prices_for_coins_data(coins_data: &CoinsData) -> Result<Vec<PriceInfo>> {
//     let mut result_prices: Vec<PriceInfo> = Vec::new();


//     for coin_def in coins_data.coins.iter() {
//         if let Some(coin_filter) = filter.coin.as_ref() {
//             if *coin_filter != coin_def.code {
//                 continue;
//             }
//         }

//         //println!("getting info for coin: {}", coin_def.code);
//         let price_provider = get_price_provider(&coin_def.price_provider);
//         let coin_result = price_provider.get_price(
//             &coin_def.code,
//             &coin_def.price_provider_data
//         ).await;

//         if coin_result.is_err() {
//             println!("Error reading coin: {} skipping", coin_def.code);
//             continue;
//         }

//         let mut coin_result = coin_result.unwrap();
//         println!("Price: {}:{:?}", coin_def.code, coin_result);
//         result_prices.append(&mut coin_result);
//     }

//     Ok(result_prices)
// }


