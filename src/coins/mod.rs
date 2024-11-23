use crate::model::PriceInfo;
use crate::provider::get_price_provider;
use crate::Result;
use crate::config::coins::{ read_default_coins_config, CoinsData };


pub async fn get_all_coins_prices(coin: &Option<String>) -> Result<Vec<PriceInfo>> {
    let coins_data: CoinsData = read_default_coins_config()?;

    get_coins_prices_for_coins_data(coin, coins_data).await
}

async fn get_coins_prices_for_coins_data(coin: &Option<String>, coins_data: CoinsData) -> Result<Vec<PriceInfo>> {
    let mut result_prices: Vec<PriceInfo> = Vec::new();
    for coin_def in coins_data.coins {
        if let Some(coin_filter) = coin.as_ref() {
            if *coin_filter != coin_def.code {
                continue;
            }
        }

        //println!("getting info for coin: {}", coin_def.code);
        let price_provider = get_price_provider(coin_def.price_provider);
        let coin_result = price_provider.get_price(
            &coin_def.code,
            coin_def.price_provider_data
        ).await;

        if coin_result.is_err() {
            println!("Error reading coin: {} skipping", coin_def.code);
            continue;
        }

        let mut coin_result = coin_result.unwrap();
        println!("Price: {}:{:?}", coin_def.code, coin_result);
        result_prices.append(&mut coin_result);
    }

    Ok(result_prices)
}
