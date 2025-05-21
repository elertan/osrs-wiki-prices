use crate::types::ItemId;
use chrono::{DateTime, Utc};
use std::collections::HashMap;

pub mod five_minutes;
pub mod one_hour;

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceItem {
    pub avg_high_price: Option<u32>,
    pub avg_low_price: Option<u32>,
    pub high_price_volume: u32,
    pub low_price_volume: u32,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricesResponseSuccess {
    pub data: HashMap<ItemId, PriceItem>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PricesResponseError {
    pub error: String,
}

#[derive(Debug, Clone, serde::Deserialize)]
#[serde(untagged)]
pub enum PricesResponse {
    Success(PricesResponseSuccess),
    Error(PricesResponseError),
}

#[cfg(test)]
mod tests {
    use crate::endpoints::prices::{PricesResponse, PricesResponseSuccess};

    #[test]
    fn parse_test_success() {
        let json = r#"{"data":{"1":{"avgHighPrice":171,"highPriceVolume":33644,"avgLowPrice":169,"lowPriceVolume":13925}},"timestamp":1697059200}"#;
        let result: PricesResponseSuccess = serde_json::from_str(json).unwrap();
        assert_eq!(result.data.len(), 1);
    }

    #[test]
    fn parse_test_untagged() {
        let json = r#"{"data":{"1":{"avgHighPrice":171,"highPriceVolume":33644,"avgLowPrice":169,"lowPriceVolume":13925}},"timestamp":1697059200}"#;
        let result: PricesResponse = serde_json::from_str(json).unwrap();
        match result {
            PricesResponse::Success(success) => {
                assert_eq!(success.data.len(), 1);
                assert_eq!(success.timestamp.timestamp(), 1697059200);
            }
            _ => panic!("Expected a success response"),
        }
    }

    #[test]
    fn parse_test_error() {
        let json = r#"{"error":"Invalid request"}"#;
        let result: PricesResponse = serde_json::from_str(json).unwrap();
        match result {
            PricesResponse::Error(error) => {
                assert_eq!(error.error, "Invalid request");
            }
            _ => panic!("Expected an error response"),
        }
    }
}