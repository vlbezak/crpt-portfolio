use std::collections::HashMap;

use crate::{client::coingecko::Coin, coins::filestore::write_data_json_to_file, config::{coins::read_default_coins_config, read_json_config}, Result};

pub fn update_coins_with_list_file() -> Result<()> {
    println!("Updating coins with ATH data");
    
    let coingecko_list_coins = read_json_config::<Vec<Coin>>("data/list.json")?;

    let mut coins_config = read_default_coins_config()?;
    let mut not_found_coins = Vec::new();

    for coin in coins_config.coins.iter_mut() {
        let coin_code = &coin.code;
        let coingecko_coin = coingecko_list_coins.iter().find(|c| c.symbol.to_uppercase() == *coin_code);
        match coingecko_coin {
            None => {
                println!("Coin {} not found in list", coin_code);
                not_found_coins.push(coin_code);
            },
            Some(c) => {
                println!("Coin {} found in list", coin_code);
                let mut ath_data = HashMap::new();
                ath_data.insert("token_id".to_string(), c.id.clone()); 
                coin.ath_provider_data = Some(ath_data);
            }
        } 
    }

    println!("Writing updated coins config");
    println!("Not found coins: {:?}", not_found_coins);
    write_data_json_to_file("data/list_new.json", &coins_config)?;

    Ok(())
}