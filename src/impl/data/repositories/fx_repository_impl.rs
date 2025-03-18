use std::path::PathBuf;

use async_trait::async_trait;
use chrono::NaiveDate;
use fractic_server_error::ServerError;
use iso_currency::Currency;

use crate::{
    data::datasources::{
        cache_datasource::{CacheDatasource, CacheDatasourceImpl},
        open_exchange_rates_datasource::{
            OpenExchangeRatesDatasource, OpenExchangeRatesDatasourceImpl,
        },
    },
    domain::repositories::fx_repository::FxRepository,
    entities::RateTable,
    errors::RateMissingForCurrency,
};

enum ApiDatasource<A1> {
    OpenExchangeRates(A1),
}

pub(crate) struct FxRepositoryImpl<
    C = CacheDatasourceImpl,              // Default.
    A1 = OpenExchangeRatesDatasourceImpl, // Default.
> where
    C: CacheDatasource,
    A1: OpenExchangeRatesDatasource,
{
    cache_datasource: C,
    api_datasource: ApiDatasource<A1>,
}

#[async_trait]
impl<C, A1> FxRepository for FxRepositoryImpl<C, A1>
where
    C: CacheDatasource,
    A1: OpenExchangeRatesDatasource,
{
    async fn rate_table(&self, date: NaiveDate) -> Result<RateTable, ServerError> {
        match self.cache_datasource.get(date).await? {
            Some(cached) => Ok(cached),
            None => {
                let fetched = match &self.api_datasource {
                    ApiDatasource::OpenExchangeRates(api) => api.historical(date).await?.into(),
                };
                self.cache_datasource.set(date, &fetched).await?;
                Ok(fetched)
            }
        }
    }

    async fn convert(
        &self,
        date: NaiveDate,
        from: Currency,
        to: Currency,
        amount: f64,
    ) -> Result<f64, ServerError> {
        let rate_table = self.rate_table(date).await?;
        let in_rate = rate_table
            .rates
            .get(&from)
            .ok_or_else(|| RateMissingForCurrency::new(&from))?;
        let out_rate = rate_table
            .rates
            .get(&to)
            .ok_or_else(|| RateMissingForCurrency::new(&to))?;
        Ok(amount * out_rate / in_rate)
    }
}

impl FxRepositoryImpl {
    pub(crate) fn using_open_exchange_rates_api(
        app_id: impl Into<String>,
        cache_dir: impl Into<PathBuf>,
    ) -> Result<Self, ServerError> {
        Ok(Self {
            cache_datasource: CacheDatasourceImpl::new(cache_dir)?,
            api_datasource: ApiDatasource::OpenExchangeRates(OpenExchangeRatesDatasourceImpl::new(
                app_id,
            )?),
        })
    }
}
