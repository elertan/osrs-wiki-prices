use crate::endpoints::prices::PricesResponse;
use crate::Client;

#[derive(Debug, thiserror::Error)]
pub enum PricesOneHourError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub trait PricesOneHourEndpoint {
    fn prices_one_hour(&self) -> impl Future<Output=Result<PricesResponse, PricesOneHourError>>;
}

impl PricesOneHourEndpoint for Client {
    async fn prices_one_hour(&self) -> Result<PricesResponse, PricesOneHourError> {
        let url = format!("{}/1h", self.base_url);
        let response = self.http_client.get(url).send().await?;
        let result = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_prices_one_hour() {
        // let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        // let result = client.prices_one_hour().await;
        // assert!(result.is_ok());
        // let prices = result.unwrap();
        // assert!(prices.timestamp.timestamp() > 0);
        // assert!(!prices.data.is_empty());
        // for (item_id, price_item) in &prices.data {
        //     assert!(item_id.id() > 0);
        //     if let Some(avg_high_price) = price_item.avg_high_price {
        //         assert!(avg_high_price > 0);
        //     }
        //     if let Some(avg_low_price) = price_item.avg_low_price {
        //         assert!(avg_low_price > 0);
        //     }
        // }
    }
}
