use std::collections::HashMap;

use chrono::NaiveDate;
use iso_currency::Currency;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RateTable {
    pub date: NaiveDate,
    pub rates: HashMap<Currency, f64>,
}
