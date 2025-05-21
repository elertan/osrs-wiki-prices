use crate::types::ItemId;
use crate::Client;


#[derive(Debug, Clone, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MappingItem {
    pub examine: String,
    pub id: ItemId,
    pub members: bool,
    #[serde(rename = "lowalch")]
    pub low_alch: Option<u32>,
    pub limit: Option<u32>,
    pub value: Option<u32>,
    #[serde(rename = "highalch")]
    pub high_alch: Option<u32>,
    pub icon: String,
    pub name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum MappingError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

pub trait MappingEndpoint {
    fn mapping(&self) -> impl Future<Output=Result<Vec<MappingItem>, MappingError>>;
}

impl MappingEndpoint for Client {
    async fn mapping(&self) -> Result<Vec<MappingItem>, MappingError> {
        let url = format!("{}/mapping", self.base_url);
        let response = self.http_client.get(url).send().await?;
        let result = response.json().await?;
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoints::mapping::MappingEndpoint;
    use crate::testing::get_test_client;
    use crate::ApiEndpoint;

    #[tokio::test]
    async fn test_mapping() {
        let client = get_test_client(ApiEndpoint::OldSchoolRuneScape);
        let result = client.mapping().await;
        assert!(result.is_ok());
        let mapping = result.unwrap();
        assert!(!mapping.is_empty());
        for item in mapping {
            assert!(item.id.id() > 0);
            assert!(!item.name.is_empty());
            assert!(!item.examine.is_empty());
            assert!(!item.icon.is_empty());
        }
    }
}
