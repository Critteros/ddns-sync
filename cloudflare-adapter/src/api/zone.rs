use super::dto::ZoneInfo;
use std::collections::HashMap;

use crate::error::ApiError;

pub type ZoneInfoList = Vec<ZoneInfo>;

pub trait ZoneInfoApi {
    const ZONE_INFO_LIST_ENDPOINT: &'static str = "https://api.cloudflare.com/client/v4/zones";
    async fn list_zones(&self) -> Result<ZoneInfoList, ApiError>;
}

impl ZoneInfoApi for crate::CloudflareClient {
    async fn list_zones(&self) -> Result<ZoneInfoList, ApiError> {
        let response = self
            .client
            .get(Self::ZONE_INFO_LIST_ENDPOINT)
            .send()
            .await?;

        let mut response_data = response
            .json::<HashMap<String, serde_json::Value>>()
            .await?;
        let zone_info_list: ZoneInfoList = serde_json::from_value(
            response_data
                .remove("result")
                .ok_or(ApiError::InvalidResponse)?,
        )?;

        Ok(zone_info_list)
    }
}
