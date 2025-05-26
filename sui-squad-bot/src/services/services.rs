use reqwest::Client;
use sui_squad_core::helpers::dtos::{AuthRequest, User};
use anyhow::Result;

use super::dto::Endpoints;

#[derive(Clone)]
pub struct Services {
    client: Client
}

impl Services {
    pub fn new() -> Self {
        let client = Client::new();

        Self { client }
    }


    pub async fn auth(&self, auth_request: AuthRequest) -> Result<User> {
        let response = self.client.post(Endpoints::Auth.to_string())
            .json(&auth_request)
            .send()
            .await?;

        let user: User = response.json().await?;

        Ok(user)
    }

}