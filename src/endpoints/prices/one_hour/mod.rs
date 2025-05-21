use crate::endpoints::prices::{PricesResponse, PricesResponseSuccess};
use crate::Client;
use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum PricesOneHourError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    Error(String),
}

#[derive(Debug, thiserror::Error)]
pub enum PricesOneHourWithTimestampError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub trait PricesOneHourEndpoint {
    fn prices_one_hour(&self) -> impl Future<Output=Result<PricesResponseSuccess, PricesOneHourError>>;
    fn prices_one_hour_with_timestamp(&self, timestamp: DateTime<Utc>) -> impl Future<Output=Result<PricesResponseSuccess, PricesOneHourError>>;
}

impl PricesOneHourEndpoint for Client {
    async fn prices_one_hour(&self) -> Result<PricesResponseSuccess, PricesOneHourError> {
        let url = format!("{}/5m", self.base_url);
        let response = self.http_client.get(url).send().await?;
        let result: PricesResponse = response.json().await?;
        match result {
            PricesResponse::Success(success) => Ok(success),
            PricesResponse::Error(error) => Err(PricesOneHourError::Error(error.error)),
        }
    }

    async fn prices_one_hour_with_timestamp(&self, timestamp: DateTime<Utc>) -> Result<PricesResponseSuccess, PricesOneHourError> {
        let ts_seconds = timestamp.timestamp();
        let url = format!("{}/5m?timestamp={}", self.base_url, ts_seconds);
        let response = self.http_client.get(url).send().await?;
        let result: PricesResponse = response.json().await?;
        match result {
            PricesResponse::Success(success) => Ok(success),
            PricesResponse::Error(error) => Err(PricesOneHourError::Error(error.error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::prices::one_hour::PricesOneHourEndpoint;
    use crate::testing::get_test_client;
    use crate::ApiEndpoint;
    use chrono::{TimeZone, Utc};

    #[tokio::test]
    async fn test_prices_one_hour() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        let result = client.prices_one_hour().await;
        assert!(result.is_ok());
        let prices = result.unwrap();
        assert!(prices.timestamp.timestamp() > 0);
        assert!(!prices.data.is_empty());
        for (item_id, price_item) in &prices.data {
            assert!(item_id.id() > 0);
            if let Some(avg_high_price) = price_item.avg_high_price {
                assert!(avg_high_price > 0);
            }
            if let Some(avg_low_price) = price_item.avg_low_price {
                assert!(avg_low_price > 0);
            }
        }
    }

    #[tokio::test]
    async fn test_prices_one_hour_with_timestamp() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        // timestamp should be divisible by 300 seconds (5 minutes), so 5-minute intervals
        let now = Utc::now();
        let one_hour_ago = now - chrono::Duration::minutes(5);
        let one_hour_ago_ts = one_hour_ago.timestamp();
        let timestamp = one_hour_ago_ts - (one_hour_ago_ts % 300);
        let date_time = Utc.timestamp_opt(timestamp, 0).single().expect("Invalid timestamp");

        let result = client.prices_one_hour_with_timestamp(date_time).await;
        assert!(result.is_ok());
        let prices = result.unwrap();
        assert!(prices.timestamp.timestamp() > 0);
        assert!(!prices.data.is_empty());
        for (item_id, price_item) in &prices.data {
            assert!(item_id.id() > 0);
            if let Some(avg_high_price) = price_item.avg_high_price {
                assert!(avg_high_price > 0);
            }
            if let Some(avg_low_price) = price_item.avg_low_price {
                assert!(avg_low_price > 0);
            }
        }
    }
}
