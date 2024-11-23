
use reqwest::{header::{HeaderMap, HeaderValue, CONTENT_TYPE}, Client};
use serde::Deserialize;
use std::env;
use crate::Result;

const API_KEY_ENV_PARAM: &str  = "COINGECKO_API_KEY";

