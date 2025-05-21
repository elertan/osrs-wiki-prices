use std::borrow::Cow;
use std::fmt::{Display, Formatter};

pub mod endpoints;
pub mod types;
#[cfg(test)]
pub mod testing;

const BASE_URL: &str = "prices.runescape.wiki/api/v1";

pub struct Client {
    http_client: reqwest::Client,
    api_endpoint: ApiEndpoint,
    base_url: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ClientNewError {
    #[error(transparent)]
    ReqwestError(#[from] reqwest::Error),
}

impl Client {
    pub fn try_new(user_agent: Cow<str>, api_endpoint: ApiEndpoint) -> Result<Self, ClientNewError> {
        let http_client = reqwest::Client::builder()
            .user_agent(user_agent.as_ref())
            .build()?;
        let base_url = format!("https://{}/{}", BASE_URL, api_endpoint);
        Ok(Self { http_client, api_endpoint, base_url })
    }
}

#[derive(Debug, Clone, Copy)]
#[derive(PartialEq)]
pub enum ApiEndpoint {
    OldSchoolRuneScape,
    DeadmanArmageddon,
}

impl Display for ApiEndpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiEndpoint::OldSchoolRuneScape => write!(f, "osrs"),
            ApiEndpoint::DeadmanArmageddon => write!(f, "dmm"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::borrow::Cow;

    #[test]
    fn test_client_new() {
        let user_agent = Cow::Borrowed("test_user_agent");
        let api_endpoint = ApiEndpoint::OldSchoolRuneScape;
        let client = Client::try_new(user_agent, api_endpoint);
        assert!(client.is_ok());
        let client = client.unwrap();
        assert_eq!(client.api_endpoint, api_endpoint);
        assert_eq!(client.base_url, format!("https://{}/{}", BASE_URL, api_endpoint));
    }
}