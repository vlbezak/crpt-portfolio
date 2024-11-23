use std::collections::HashMap;

use crate::{
    config::wallets::WalletsData,
    model::{ Currency, PriceInfo, ReportOrder, ReportSortBy },
};
use ordered_float::OrderedFloat;

#[derive(Debug)]
pub struct ReportFilter {
    coin: Option<String>,
    wallet_name: Option<String>,
    wallet_kind: Option<String>,
    wallet_address: Option<String>,
    currency: Currency,
    group_by_token: bool,
    sort_by: ReportSortBy,
    order: ReportOrder,
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
    wallet_name: String,
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
            let Some(val_of_coin) = find_price(&holding.coin, &prices, &filter.currency) else {
                println!("Cannot find price for {}", holding.coin);
                continue;
            };
            let val_of_coin = holding.amount * val_of_coin;

            report_lines.push(ReportLine {
                token: holding.coin.clone(),
                amount: holding.amount,
                value: val_of_coin,
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
    let mut grouped: HashMap<String, (f64, f64)> = HashMap::new();

    for line in report_lines {
        let entry = grouped.entry(line.token.clone()).or_insert((0.0, 0.0));
        entry.0 += line.amount;
        entry.1 += line.value;
    }

    grouped
        .into_iter()
        .map(|(token, (total_amount, total_value))| ReportLine {
            token,
            amount: total_amount,
            value: total_value,
            wallet_name: "-".to_string(),
            wallet_kind: "-".to_string(),
            wallet_address: "-".to_string(),
        })
        .collect()
}

pub fn write_report(report_lines: &Vec<ReportLine>) {
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

    let mut sum = 0.0;
    let mut amount = 0.0;
    for line in report_lines {
        sum += line.value;
        amount += line.amount;

        println!(
            "{:8}|{:14.4} | {:12.4} | {:32} | {:32}",
            line.token,
            line.amount,
            line.value,
            line.wallet_name,
            line.wallet_address
        );
    }

    println!(
        "---------------------------------------------------------------------------------------------"
    );
    println!("Amount  | {:12.4}  |", amount);
    println!("Sum     | {:12.4}  |", sum);
    println!(
        "-----------------------------------------------------------------------------------------------"
    );
}

fn find_price(coin: &str, prices: &Vec<PriceInfo>, currency: &Currency) -> Option<f64> {
    prices
        .into_iter()
        .find(|price| price.coin == coin && price.currency == *currency)
        .map(|price| price.value)
}
