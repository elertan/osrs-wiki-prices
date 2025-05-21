use crate::types::ItemId;
use crate::Client;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestResponse {
    pub data: HashMap<ItemId, LatestItem>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LatestItem {
    pub high: Option<u32>,
    pub low: Option<u32>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub high_time: Option<DateTime<Utc>>,
    #[serde(with = "chrono::serde::ts_seconds_option")]
    pub low_time: Option<DateTime<Utc>>,
}

#[derive(Debug, thiserror::Error)]
pub enum LatestError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub trait LatestEndpoint {
    fn latest(&self) -> impl Future<Output=Result<HashMap<ItemId, LatestItem>, LatestError>>;
}

impl LatestEndpoint for Client {
    async fn latest(&self) -> Result<HashMap<ItemId, LatestItem>, LatestError> {
        let url = format!("{}/latest", self.base_url);
        let response = self.http_client.get(url).send().await?;
        let result: LatestResponse = response.json().await?;
        Ok(result.data)
    }
}

mod tests {
    use crate::endpoints::latest::LatestEndpoint;
    use crate::testing::get_test_client;
    use crate::ApiEndpoint;

    #[tokio::test]
    async fn test_latest() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        let result = client.latest().await;
        assert!(result.is_ok());
        let data = result.unwrap();
        assert!(!data.is_empty());
        for (item_id, item) in data.iter() {
            assert!(item_id.id() > 0);
            assert!(item.high.is_some() || item.low.is_some());
            if let Some(high_time) = item.high_time {
                assert!(high_time.timestamp() > 0);
            }
            if let Some(low_time) = item.low_time {
                assert!(low_time.timestamp() > 0);
            }
            if let Some(high) = item.high {
                assert!(high > 0);
            }
            if let Some(low) = item.low {
                assert!(low > 0);
            }
        }
    }
}
