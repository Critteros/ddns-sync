use super::dto::DnsEntry;
use crate::error::ApiError;
use dyn_fmt::AsStrFormatExt;
use std::collections::HashMap;

type DnsRecords = Vec<DnsEntry>;

pub trait DnsLookupApi {
    const DNS_RECORDS_FOR_ZONE_URL: &'static str =
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records";
    async fn dns_records_for_zone(&self, zone_id: &str) -> Result<DnsRecords, ApiError>;
}

impl DnsLookupApi for crate::CloudflareClient {
    async fn dns_records_for_zone(&self, zone_id: &str) -> Result<DnsRecords, ApiError> {
        let url = Self::DNS_RECORDS_FOR_ZONE_URL.format(&[zone_id]);
        let response = self.client.get(&url).send().await?;
        let mut records: HashMap<String, serde_json::Value> = response.json().await?;

        let dns_record: DnsRecords = serde_json::from_value(
            records
                .remove("result")
                .ok_or_else(|| ApiError::InvalidResponse)?,
        )?;
        return Ok(dns_record);
    }
}
