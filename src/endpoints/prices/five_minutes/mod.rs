use crate::endpoints::prices::{PricesResponse, PricesResponseSuccess};
use crate::Client;
use chrono::{DateTime, Utc};

#[derive(Debug, thiserror::Error)]
pub enum PricesFiveMinutesError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
    #[error("{0}")]
    Error(String),
}

#[derive(Debug, thiserror::Error)]
pub enum PricesFiveMinutesWithTimestampError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub trait PricesFiveMinutesEndpoint {
    fn prices_five_minutes(&self) -> impl Future<Output=Result<PricesResponseSuccess, PricesFiveMinutesError>>;
    fn prices_five_minutes_with_timestamp(&self, timestamp: DateTime<Utc>) -> impl Future<Output=Result<PricesResponseSuccess, PricesFiveMinutesError>>;
}

impl PricesFiveMinutesEndpoint for Client {
    async fn prices_five_minutes(&self) -> Result<PricesResponseSuccess, PricesFiveMinutesError> {
        let url = format!("{}/5m", self.base_url);
        let response = self.http_client.get(url).send().await?;
        let result: PricesResponse = response.json().await?;
        match result {
            PricesResponse::Success(success) => Ok(success),
            PricesResponse::Error(error) => Err(PricesFiveMinutesError::Error(error.error)),
        }
    }

    async fn prices_five_minutes_with_timestamp(&self, timestamp: DateTime<Utc>) -> Result<PricesResponseSuccess, PricesFiveMinutesError> {
        let ts_seconds = timestamp.timestamp();
        let url = format!("{}/5m?timestamp={}", self.base_url, ts_seconds);
        let response = self.http_client.get(url).send().await?;
        let result: PricesResponse = response.json().await?;
        match result {
            PricesResponse::Success(success) => Ok(success),
            PricesResponse::Error(error) => Err(PricesFiveMinutesError::Error(error.error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::prices::five_minutes::PricesFiveMinutesEndpoint;
    use crate::testing::get_test_client;
    use crate::ApiEndpoint;
    use chrono::{TimeZone, Utc};

    #[tokio::test]
    async fn test_prices_five_minutes() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        let result = client.prices_five_minutes().await;
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
    async fn test_prices_five_minutes_with_timestamp() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        // timestamp should be divisible by 300 seconds (5 minutes), so 5-minute intervals
        let now = Utc::now();
        let five_minutes_ago = now - chrono::Duration::minutes(5);
        let five_minutes_ago_ts = five_minutes_ago.timestamp();
        let timestamp = five_minutes_ago_ts - (five_minutes_ago_ts % 300);
        let date_time = Utc.timestamp_opt(timestamp, 0).single().expect("Invalid timestamp");

        let result = client.prices_five_minutes_with_timestamp(date_time).await;
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
