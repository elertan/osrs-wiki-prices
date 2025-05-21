use crate::types::ItemId;
use crate::Client;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Copy)]
pub enum Timestep {
    FiveMinutes,
    OneHour,
    SixHours,
    OneDay,
}

impl AsRef<str> for Timestep {
    fn as_ref(&self) -> &str {
        match self {
            Timestep::FiveMinutes => "5m",
            Timestep::OneHour => "1h",
            Timestep::SixHours => "6h",
            Timestep::OneDay => "24h",
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesItem {
    pub avg_high_price: Option<u32>,
    pub avg_low_price: Option<u32>,
    pub high_price_volume: u32,
    pub low_price_volume: u32,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}


#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimeseriesResponse {
    pub data: Vec<TimeseriesItem>,
}

#[derive(Debug, thiserror::Error)]
pub enum TimeseriesError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub trait TimeseriesEndpoint {
    fn timeseries(&self, id: ItemId, timestep: Timestep) -> impl Future<Output=Result<Vec<TimeseriesItem>, TimeseriesError>>;
}

impl TimeseriesEndpoint for Client {
    async fn timeseries(&self, id: ItemId, timestep: Timestep) -> Result<Vec<TimeseriesItem>, TimeseriesError> {
        let url = format!("{}/timeseries?timestep={}&id={}", self.base_url, timestep.as_ref(), id.id());
        let response = self.http_client.get(url).send().await?;
        let result: TimeseriesResponse = response.json().await?;
        Ok(result.data)
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::timeseries::{TimeseriesEndpoint, Timestep};
    use crate::testing::get_test_client;
    use crate::types::ItemId;
    use crate::ApiEndpoint;

    #[tokio::test]
    async fn test_timeseries() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        let result = client.timeseries(ItemId::new(4151), Timestep::FiveMinutes).await;
        assert!(result.is_ok());
        let timeseries = result.unwrap();
        assert!(!timeseries.is_empty());
        for item in timeseries {
            assert!(item.timestamp.timestamp() > 0);
        }
    }
}
