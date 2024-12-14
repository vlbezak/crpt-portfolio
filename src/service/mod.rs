use std::collections::HashMap;

use crate::{
    config::wallets::WalletsData,
    model::{ Currency, PriceInfo, ReportOrder, ReportSortBy },
};
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct ReportFilter {
    pub coin: Option<String>,
    pub wallet_name: Option<String>,
    pub wallet_kind: Option<String>,
    pub wallet_address: Option<String>,
    pub currency: Currency,
    pub group_by_token: bool,
    pub sort_by: ReportSortBy,
    pub order: ReportOrder,
}

impl Default for ReportFilter {
    fn default() -> Self {
        Self {
            coin: None,
            wallet_name: None,
            wallet_kind: None,
            wallet_address: None,
            currency: Currency::USD,
            group_by_token: false,
            sort_by: ReportSortBy::Value,
            order: ReportOrder::Desc,
        }
    }
}

impl ReportFilter {
    pub fn new(
        coin: Option<String>,
        wallet_name: Option<String>,
        wallet_kind: Option<String>,
        wallet_address: Option<String>,
        currency: Currency,
        group_by_token: bool,
        sort_by: ReportSortBy,
        order: ReportOrder
    ) -> Self {
        Self {
            coin,
            wallet_name,
            wallet_kind,
            wallet_address,
            currency,
            group_by_token,
            sort_by,
            order,
        }
    }
}

pub struct ReportLine {
    token: String,
    amount: f64,
    value: f64,
    mkt_cap: f64,
    wallet_name: String,
    #[allow(unused)]
    wallet_kind: String,
    wallet_address: String,
}

pub struct ListWalletLine {
    wallet_name: String,
    #[allow(unused)]
    wallet_kind: String,
    wallet_address: String,
}

pub fn report_holdings(
    wallets_data: &WalletsData,
    prices: &Vec<PriceInfo>,
    filter: &ReportFilter
) -> Vec<ReportLine> {
    let mut report_lines = Vec::new();

    for wallet in wallets_data.wallets.iter() {
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
        for holding in wallet.holdings.iter() {
            if let Some(coin) = filter.coin.as_ref() {
                if holding.coin != *coin {
                    continue;
                }
            }
            let Some(price_info) = find_price_info(&holding.coin, &prices, &filter.currency) else {
                println!("Cannot find price for {}", holding.coin);
                continue;
            };
            let val_of_coin = holding.amount * price_info.value;

            report_lines.push(ReportLine {
                token: holding.coin.clone(),
                amount: holding.amount,
                value: val_of_coin,
                mkt_cap: price_info.market_cap,
                wallet_name: wallet.name.clone(),
                wallet_kind: wallet.kind.clone(),
                wallet_address: wallet.address.clone(),
            });
        }
    }

    if filter.group_by_token {
        report_lines = group_by_token(&report_lines);
    }

    match filter.sort_by {
        ReportSortBy::Value => { report_lines.sort_by_key(|line| OrderedFloat(line.value)) },
        ReportSortBy::Amount => { report_lines.sort_by_key(|line| OrderedFloat(line.amount)) },
        ReportSortBy::Token => { report_lines.sort_by(|a,b| a.token.cmp(&b.token)) },
    }

    if filter.order == ReportOrder::Desc {
        report_lines.reverse();
    }

    report_lines
}

fn group_by_token(report_lines: &Vec<ReportLine>) -> Vec<ReportLine> {
    let mut grouped: HashMap<String, (f64, f64, f64)> = HashMap::new();

    for line in report_lines {
        let entry = grouped.entry(line.token.clone()).or_insert((0.0, 0.0, 0.0));
        entry.0 += line.amount;
        entry.1 += line.value;
        entry.2 = line.mkt_cap;
    }

    grouped
        .into_iter()
        .map(|(token, (total_amount, total_value, mkt_cap))| ReportLine {
            token,
            amount: total_amount,
            value: total_value,
            mkt_cap,
            wallet_name: "-".to_string(),
            wallet_kind: "-".to_string(),
            wallet_address: "-".to_string(),
        })
        .collect()
}

pub fn write_report(report_lines: &Vec<ReportLine>) {
    println!(
        "-------------------------------------------------------------------------------------------------------"
    );
    println!(
        "{:8}|{:14.6} | {:12.2} | {:22.2} | {:32} | {:32}",
        "Token",
        "Amount",
        "Value",
        "Mkt.Cap",
        "Wallet",
        "Address"
    );
    println!(
        "-------------------------------------------------------------------------------------------------------"
    );

    let mut sum = 0.0;
    let mut amount = 0.0;
    for line in report_lines {
        sum += line.value;
        amount += line.amount;

        println!(
            "{:8}|{:14.6} | {:12.2} | {:22.2} | {:32} | {:32}",
            line.token,
            line.amount,
            line.value,
            line.mkt_cap,
            line.wallet_name,
            line.wallet_address
        );
    }

    println!(
        "-----------------------------------------------------------------------------------------------------"
    );
    println!("Amount  | {:14.6}  |", amount);
    println!("Sum     | {:14.2}  |", sum);
    println!(
        "-------------------------------------------------------------------------------------------------------"
    );
}

fn find_price_info<'a>(coin: &str, prices: &'a Vec<PriceInfo>, currency: &Currency) -> Option<&'a PriceInfo> {
    prices
        .iter()
        .find(|price| price.coin == coin && price.currency == *currency)
        //.map(|price| price.value)
}

pub fn list_wallets(wallets_data: &WalletsData) -> Vec<ListWalletLine> {
    
    let mut wallets = Vec::new();
    
    for wallet in wallets_data.wallets.iter() {
        let line = ListWalletLine {
            wallet_name: wallet.name.clone(),
            wallet_kind: wallet.kind.clone(),
            wallet_address: wallet.address.clone(),
        };
        wallets.push(line);
    }

    wallets

}

pub fn write_wallets_report(report_lines: &Vec<ListWalletLine>) {
    println!(
        "-----------------------------------------------------------------------------------------------"
    );
    println!(
        "{:32} | {:32} | {:64}",
        "Wallet",
        "Kind",
        "Address"
    );
    println!(
        "-----------------------------------------------------------------------------------------------"
    );

    for line in report_lines {
        println!(
            "{:32} | {:32} | {:64}",
            line.wallet_name,
            line.wallet_kind,
            line.wallet_address
        );
    }

    println!(
        "---------------------------------------------------------------------------------------------"
    );
    println!(
        "-----------------------------------------------------------------------------------------------"
    );
}
