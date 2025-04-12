

use std::{io::{self, Write}, thread, time::Duration};

use crate::{coins::{filestore::AdditionalDataStore, DataStore}, config::coins::{read_default_coins_config, CoinsData}, model::{AthInfo, Currency}, provider::get_ath_provider};
use crate::Result;

pub async fn update_ath_data_for_token(token: &str, currencies: &Vec<Currency>) -> Result<()> {
    println!("Updating ATH data for token: {}", token);
    let data_store = AdditionalDataStore {
    };
    
    println!("Getting coins config");

    let coins_data: CoinsData = read_default_coins_config()?;
    let Some(coin_def) = coins_data.get_coin_def(token) else {
        return Err(format!("Coin {} not found", token).into());
    };
    
    let ath_provider = get_ath_provider(&coin_def.ath_provider);
    println!("Getting ATH for: {}", token);

    let ath_info = ath_provider.get_ath(&coin_def.code, coin_def.ath_provider_data.as_ref(), currencies).await?;
    println!("ATH info: {:?}", ath_info);

    data_store.write_data(&ath_info)?;

    Ok(())
}

pub async fn update_ath_data_for_all_tokens(currencies: &Vec<Currency>) -> Result<()> {
    println!("Updating ATH data for all tokens");
    let data_store = AdditionalDataStore {
    };

    println!("Getting coins config");

    let coins_data: CoinsData = read_default_coins_config()?;
    for coin_def in coins_data.coins.iter() {
        let ath_provider = get_ath_provider(&coin_def.ath_provider);
        println!("Getting ATH for: {}", coin_def.code);

        let res = ath_provider.get_ath(&coin_def.code, coin_def.ath_provider_data.as_ref(), currencies).await;
        
        match res {
            Err(e) => {
                println!("Error getting ATH for: {} - {}", coin_def.code, e);
            },
            Ok(ath_info) => {
                println!("ATH info: {:?}", ath_info);
                data_store.write_data(&ath_info)?;
            }
        }

        let mut sleep = 20;
        println!("Sleeping for {sleep} seconds");
        while sleep > 0 {
            print!(".");
            io::stdout().flush().unwrap();
            thread::sleep(Duration::from_secs(1));
            sleep -= 1;
        }
    }

    Ok(())
}