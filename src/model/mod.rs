
use std::str::FromStr;

use clap::ValueEnum;

//use core::Result;

#[derive(Debug, PartialEq, Clone, ValueEnum)]
pub enum Currency {
    EUR,
    USD
}

impl FromStr for Currency {

    type Err = String;

    fn from_str(val: &str) -> Result<Self, Self::Err> {
        match val {
        "eur" | "EUR" => Ok(Currency::EUR),
        "usd" | "USD" => Ok(Currency::USD),
        _ => Err(format!("Cannot convert {} into currency", val).into())   
        }
    }
}

#[derive(Debug)]
pub struct PriceInfo {
    pub coin: String,
    pub currency: Currency,
    pub value: f64,
}

#[derive(Debug, ValueEnum, Clone)]
pub enum ReportSortBy {
    Token,
    Amount,
    Value,
}

#[derive(Debug, ValueEnum, Clone, PartialEq)]
pub enum ReportOrder {
    Asc,
    Desc,
}