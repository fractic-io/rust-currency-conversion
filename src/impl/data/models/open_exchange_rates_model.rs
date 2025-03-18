use std::collections::HashMap;

use chrono::{serde::ts_seconds, DateTime, Utc};
use iso_currency::Currency;
use serde::{Deserialize, Serialize};

use crate::entities::RateTable;

#[derive(Debug, Serialize, Deserialize)]
pub(crate) struct OpenExchangeRatesModel {
    pub(crate) disclaimer: String,
    pub(crate) license: String,
    #[serde(with = "ts_seconds")]
    pub(crate) timestamp: DateTime<Utc>,
    pub(crate) base: Currency,
    pub(crate) rates: HashMap<Currency, f64>,
}

// --

impl Into<RateTable> for OpenExchangeRatesModel {
    fn into(self) -> RateTable {
        RateTable {
            date: self.timestamp.date_naive(),
            rates: self.rates,
        }
    }
}
