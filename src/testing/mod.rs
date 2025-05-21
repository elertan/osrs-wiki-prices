use crate::{ApiEndpoint, Client};

pub fn get_test_client(api_endpoint: ApiEndpoint) -> Client {
    let user_agent = "osrs-wiki-prices-rs";
    Client::try_new(user_agent.into(), api_endpoint).expect("Failed to create test client")
}