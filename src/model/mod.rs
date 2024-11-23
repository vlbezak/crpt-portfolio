pub mod holdings;
pub mod coin_info_config;

use crate::Result;

#[derive(Debug, PartialEq)]
pub enum Currency {
    EUR,
    USD
}

impl Currency {
    pub fn from(val: &str) -> Result<Self> {
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