use serde::Deserialize;
use serde::de;
use std::{ collections::HashMap, fs };
use crate::{ Result };

pub mod coins;
pub mod wallets;

pub fn read_json_config<T>(file_path: &str) -> Result<T> where T: de::DeserializeOwned {
    // Read the contents of the file
    println!("Reading {}", file_path);
    let content = fs
        ::read_to_string(file_path)
        .map_err(|e| format!("Cannot read coins configuration {}: {}", file_path, e))?;

    let config: T = serde_json
        ::from_str(&content)
        .map_err(|e| format!("Invalid json file {}: {}", file_path, e))?;
    
    println!("Read {}", file_path);
    Ok(config)
}
