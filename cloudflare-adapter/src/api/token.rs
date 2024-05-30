use super::client::CloudflareClient;
use crate::error::ApiError;

pub trait TokenVerify {
    const TOKEN_ENDPOINT: &'static str = "https://api.cloudflare.com/client/v4/user/tokens/verify";
    async fn verify_token(&self) -> Result<bool, ApiError>;
}

impl TokenVerify for CloudflareClient {
    async fn verify_token(&self) -> Result<bool, ApiError> {
        let request = self.client.get(Self::TOKEN_ENDPOINT).send().await?;

        if request.status().is_success() {
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
