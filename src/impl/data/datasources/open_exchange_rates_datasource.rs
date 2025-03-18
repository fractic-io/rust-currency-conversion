use async_trait::async_trait;
use chrono::NaiveDate;
use fractic_server_error::ServerError;
use serde::de::DeserializeOwned;

use crate::{
    data::models::open_exchange_rates_model::OpenExchangeRatesModel,
    errors::OpenExchangeRatesApiError,
};

#[async_trait]
pub(crate) trait OpenExchangeRatesDatasource: Send + Sync {
    /// Get the latest exchange rates available.
    /// https://docs.openexchangerates.org/reference/latest-json
    async fn latest(&self) -> Result<OpenExchangeRatesModel, ServerError>;

    /// Get historical exchange rates for any date available from the Open
    /// Exchange Rates API, currently going back to 1st January 1999.
    /// https://docs.openexchangerates.org/reference/historical-json
    async fn historical(&self, date: NaiveDate) -> Result<OpenExchangeRatesModel, ServerError>;
}

pub(crate) struct OpenExchangeRatesDatasourceImpl {
    app_id: String,
}

#[async_trait]
impl OpenExchangeRatesDatasource for OpenExchangeRatesDatasourceImpl {
    async fn latest(&self) -> Result<OpenExchangeRatesModel, ServerError> {
        let url = "https://openexchangerates.org/api/latest.json";
        self.callout(url, "Latest").await
    }

    async fn historical(&self, date: NaiveDate) -> Result<OpenExchangeRatesModel, ServerError> {
        let url = format!(
            "https://openexchangerates.org/api/historical/{}.json",
            date.format("%Y-%m-%d")
        );
        self.callout(&url, "Historical").await
    }
}

impl OpenExchangeRatesDatasourceImpl {
    pub(crate) async fn new(app_id: impl Into<String>) -> Result<Self, ServerError> {
        Ok(Self {
            app_id: app_id.into(),
        })
    }

    async fn callout<T: DeserializeOwned>(
        &self,
        url: &str,
        function_name: &str,
    ) -> Result<T, ServerError> {
        let client = reqwest::Client::new();
        let response = client
            .get(url)
            .query(&[("app_id", &self.app_id)])
            .send()
            .await
            .map_err(|e| {
                OpenExchangeRatesApiError::with_debug(function_name, "callout failed to send", &e)
            })?;

        if !response.status().is_success() {
            return Err(OpenExchangeRatesApiError::with_debug(
                function_name,
                &format!(
                    "callout returned with {} status code",
                    response.status().to_string(),
                ),
                &response.text().await.unwrap_or_default(),
            ));
        }

        response.json().await.map_err(|e| {
            OpenExchangeRatesApiError::with_debug(
                function_name,
                "failed to parse callout response",
                &e,
            )
        })
    }
}
