use crate::cloudflare::TokenLookup::{Invalid, Valid};
use crate::config::Config;
use anyhow::{anyhow, Result};
use reqwest::header;
use std::collections::HashMap;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CloudflareApi {
    api_token: String,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TokenLookup {
    Valid,
    Invalid,
}

impl CloudflareApi {
    pub fn from_config(config: &Config) -> Self {
        Self {
            api_token: config.cloudflare_api_key.clone(),
        }
    }
}

impl CloudflareApi {
    fn get_client(&self) -> Result<reqwest::Client> {
        let mut headers = header::HeaderMap::new();
        let mut auth_header_value =
            header::HeaderValue::from_str(format!("Bearer {}", &self.api_token).as_str())?;
        auth_header_value.set_sensitive(true);
        headers.insert(header::AUTHORIZATION, auth_header_value);

        Ok(reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()?)
    }

    pub async fn check_token(&self) -> Result<TokenLookup> {
        let client = self.get_client()?;
        let value = client
            .get("https://api.cloudflare.com/client/v4/user/tokens/verify")
            .send()
            .await?
            .json::<HashMap<String, serde_json::Value>>()
            .await?
            .get("success")
            .ok_or_else(|| anyhow!("No \"success\" field in API response "))?
            .as_bool()
            .ok_or_else(|| anyhow!("\"success\" field is not a boolean"))?;

        if value {
            Ok(Valid)
        } else {
            Ok(Invalid)
        }
    }
}
