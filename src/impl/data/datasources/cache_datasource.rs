use std::path::PathBuf;

use async_trait::async_trait;
use chrono::NaiveDate;
use fractic_server_error::ServerError;

use crate::{
    entities::RateTable,
    errors::{CacheIoError, CacheSerializationError},
};

#[async_trait]
pub(crate) trait CacheDatasource: Send + Sync {
    /// Get rate table for given date, if exists.
    async fn get(&self, date: NaiveDate) -> Result<Option<RateTable>, ServerError>;

    /// Set rate table for given date.
    async fn set(&self, date: NaiveDate, rate_table: &RateTable) -> Result<(), ServerError>;
}

pub(crate) struct CacheDatasourceImpl {
    cache_dir: PathBuf,
}

#[async_trait]
impl CacheDatasource for CacheDatasourceImpl {
    async fn get(&self, date: NaiveDate) -> Result<Option<RateTable>, ServerError> {
        let file_path = self.cache_dir.join(date.format("%Y-%m-%d").to_string());
        if file_path.exists() {
            let rate_table = tokio::fs::read_to_string(&file_path)
                .await
                .map_err(|e| CacheIoError::with_debug(&file_path.to_string_lossy(), &e))?;
            Ok(Some(serde_json::from_str(&rate_table).map_err(|e| {
                CacheSerializationError::with_debug(&file_path.to_string_lossy(), &e)
            })?))
        } else {
            Ok(None)
        }
    }

    async fn set(&self, date: NaiveDate, rate_table: &RateTable) -> Result<(), ServerError> {
        let file_path = self.cache_dir.join(date.format("%Y-%m-%d").to_string());
        let rate_table = serde_json::to_string(rate_table)
            .map_err(|e| CacheSerializationError::with_debug(&file_path.to_string_lossy(), &e))?;
        tokio::fs::write(&file_path, rate_table)
            .await
            .map_err(|e| CacheIoError::with_debug(&file_path.to_string_lossy(), &e))?;
        Ok(())
    }
}

impl CacheDatasourceImpl {
    pub(crate) fn new(cache_dir: impl Into<PathBuf>) -> Result<Self, ServerError> {
        Ok(Self {
            cache_dir: cache_dir.into(),
        })
    }
}
