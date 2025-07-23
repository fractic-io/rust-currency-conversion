use std::path::PathBuf;

use chrono::NaiveDate;
use fractic_server_error::ServerError;
use iso_currency::Currency;

use crate::{
    data::repositories::fx_repository_impl::FxRepositoryImpl,
    domain::repositories::fx_repository::FxRepository, errors::InvalidIsoCurrency,
    CurrencyConversionCtxView,
};

pub struct FxUtil {
    fx_repository: FxRepositoryImpl,
}

pub trait IntoCurrency {
    fn try_into(self) -> Result<Currency, ServerError>;
}
impl IntoCurrency for &str {
    fn try_into(self) -> Result<Currency, ServerError> {
        Currency::from_code(self).ok_or_else(|| InvalidIsoCurrency::new(self))
    }
}
impl IntoCurrency for String {
    fn try_into(self) -> Result<Currency, ServerError> {
        Currency::from_code(&self).ok_or_else(|| InvalidIsoCurrency::new(&self))
    }
}
impl IntoCurrency for Currency {
    fn try_into(self) -> Result<Currency, ServerError> {
        Ok(self)
    }
}

impl FxUtil {
    /// Convert an amount from one currency to another, using the exchange rate
    /// for the given date.
    ///
    /// Only makes an API call if the rate is not already cached.
    ///
    /// Sign is preserved (i.e. a negative input will result in a negative
    /// output).
    pub async fn convert(
        &self,
        date: NaiveDate,
        from: impl IntoCurrency,
        to: impl IntoCurrency,
        amount: f64,
    ) -> Result<f64, ServerError> {
        self.fx_repository
            .convert(date, from.try_into()?, to.try_into()?, amount)
            .await
    }
}

impl FxUtil {
    pub fn using_open_exchange_rates_api(
        app_id: impl Into<String>,
        cache_dir: impl Into<PathBuf>,
    ) -> Result<Self, ServerError> {
        Ok(Self {
            fx_repository: FxRepositoryImpl::using_open_exchange_rates_api(app_id, cache_dir)?,
        })
    }

    pub fn using_open_exchange_rates_api_from_secrets(
        ctx: &impl CurrencyConversionCtxView,
        cache_dir: impl Into<PathBuf>,
    ) -> Result<Self, ServerError> {
        Ok(Self {
            fx_repository: FxRepositoryImpl::using_open_exchange_rates_api(
                ctx.open_exchange_rates_app_id(),
                cache_dir,
            )?,
        })
    }
}
