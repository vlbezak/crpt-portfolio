use crate::{ config::wallets::WalletsData, model::{ Currency, PriceInfo } };

#[derive(Debug)]
pub struct SumFilter {
    coin: Option<String>,
    wallet_name: Option<String>,
    wallet_kind: Option<String>,
    wallet_address: Option<String>,
    currency: Currency,
}

impl Default for SumFilter {
    fn default() -> Self {
        Self {
            coin: None,
            wallet_name: None,
            wallet_kind: None,
            wallet_address: None,
            currency: Currency::USD,
        }
    }
}

impl SumFilter {
    pub fn new(
        coin: Option<String>,
        wallet_name: Option<String>,
        wallet_kind: Option<String>,
        wallet_address: Option<String>,
        currency: Currency
    ) -> Self {
        Self {
            coin,
            wallet_name,
            wallet_kind,
            wallet_address,
            currency,
        }
    }
}

pub fn report_holdings(wallets_data: WalletsData, prices: &Vec<PriceInfo>, filter: &SumFilter) -> f64 {
    let mut sum: f64 = 0.0;
    let mut amount: f64 = 0.0;

    println!(
        "-----------------------------------------------------------------------------------------------"
    );
    println!(
        "{:8}|{:14.6} | {:12.6} | {:32} | {:32}",
        "Token",
        "Amount",
        "Value",
        "Wallet",
        "Address"
    );
    println!(
        "-----------------------------------------------------------------------------------------------"
    );
    for wallet in wallets_data.wallets.into_iter() {
        if let Some(wallet_address) = filter.wallet_address.as_ref() {
            if wallet.address != *wallet_address {
                continue;
            }
        }
        if let Some(wallet_kind) = filter.wallet_kind.as_ref() {
            if wallet.kind != *wallet_kind {
                continue;
            }
        }
        if let Some(wallet_name) = filter.wallet_name.as_ref() {
            if wallet.name != *wallet_name {
                continue;
            }
        }
        for holding in wallet.holdings.into_iter() {
            if let Some(coin) = filter.coin.as_ref() {
                if holding.coin != *coin {
                    continue;
                }
            }
            let Some(val_of_coin) = find_price(&holding.coin, &prices, &filter.currency) else {
                println!("Cannot find price for {}", holding.coin);
                continue;
            };
            let val_of_coin = holding.amount * val_of_coin;
            println!(
                "{:8}|{:14.4} | {:12.4} | {:32} | {:32}",
                holding.coin,
                holding.amount,
                val_of_coin,
                wallet.name,
                wallet.address
            );
            sum += val_of_coin;

            if filter.coin.is_some() {
                amount += holding.amount;
            }
        }
    }
    println!(
        "-----------------------------------------------------------------------------------------------"
    );
    if filter.coin.is_some() {
        println!("Amount  | {:12.4}  |", amount);
    }

    println!("Sum     | {:12.4}  |", sum);
    println!(
        "-----------------------------------------------------------------------------------------------"
    );

    sum
}

fn find_price(coin: &str, prices: &Vec<PriceInfo>, currency: &Currency) -> Option<f64> {
    prices
        .into_iter()
        .find(|price| price.coin == coin && price.currency == *currency)
        .map(|price| price.value)
}
