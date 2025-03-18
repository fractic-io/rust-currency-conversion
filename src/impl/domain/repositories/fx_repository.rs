use async_trait::async_trait;
use chrono::NaiveDate;
use fractic_server_error::ServerError;
use iso_currency::Currency;

use crate::entities::RateTable;

#[async_trait]
pub(crate) trait FxRepository: Send + Sync {
    async fn rate_table(&self, date: NaiveDate) -> Result<RateTable, ServerError>;

    async fn convert(
        &self,
        date: NaiveDate,
        from: Currency,
        to: Currency,
        amount: f64,
    ) -> Result<f64, ServerError>;
}
