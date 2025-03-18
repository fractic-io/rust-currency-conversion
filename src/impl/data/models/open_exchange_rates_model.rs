use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use iso_currency::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenExchangeRatesModel {
    pub(crate) disclaimer: String,
    pub(crate) license: String,
    #[serde(with = "ts_seconds")]
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) base: Currency,
    pub(crate) rates: HashMap<String, f64>,
}
