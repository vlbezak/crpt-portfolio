use std::collections::HashMap;

use crate::coins::CoinPriceStore;
use crate::config::coins::{read_default_coins_config, CoinDef, CoinsData, PriceProviderEnum};
use crate::model::{Currency, PriceInfo};
use crate::provider::get_price_provider;
use crate::Result;
use crate::coins::filestore::CoinPriceFileStore;

pub async fn update_coins_prices(currencies: &Vec<Currency>) -> Result<()> {
    let coin_price_store = CoinPriceFileStore {
        dir_name: String::from("data"),
    };
    
    println!("Getting coins config");

    let coins_data: CoinsData = read_default_coins_config()?;
    let price_info = get_coins_prices_for_coins_data(&coins_data, currencies).await?;

    println!("Storing prices: {}", price_info.len());
    coin_price_store.write_prices(&price_info)?;
    Ok(())
}

async fn get_coins_prices_for_coins_data(coins_data: &CoinsData, currencies: &Vec<Currency>) -> Result<Vec<PriceInfo>> {
    let mut result_prices: Vec<PriceInfo> = Vec::new();

    let mut providers: HashMap<&PriceProviderEnum, Vec<CoinDef>> = HashMap::new();
    for coin_def in coins_data.coins.iter() {
        providers.entry(&coin_def.price_provider)
        .or_insert_with(Vec::new)
        .push(coin_def.clone());
    }

    println!("Providers: {:?}", providers.keys());
    for (provider, coins) in providers {
        println!("Processing provider:{:?}", provider);
        let provider_impl = get_price_provider(&provider);
        let mut prices = provider_impl.get_prices(&coins, &currencies).await?;
        result_prices.append(&mut prices);
    }

    //println!("Result prices: {:#?}", result_prices);

    Ok(result_prices)
}