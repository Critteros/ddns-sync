use super::dto::DnsEntry;
use crate::error::ApiError;
use dyn_fmt::AsStrFormatExt;
use std::collections::HashMap;
use std::net::Ipv4Addr;

type DnsRecords = Vec<DnsEntry>;

#[derive(Debug, PartialEq, Eq)]
pub struct DnsARecord {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub name: String,
    pub ip: Ipv4Addr,
    pub comment: Option<String>,
}

pub trait DnsLookupApi {
    const DNS_RECORDS_FOR_ZONE_URL: &'static str =
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records";
    #[allow(non_snake_case)]
    async fn dns_A_records_for_zone(&self, zone_id: &str) -> Result<Vec<DnsARecord>, ApiError>;
}

impl DnsLookupApi for crate::CloudflareClient {
    #[allow(non_snake_case)]
    async fn dns_A_records_for_zone(&self, zone_id: &str) -> Result<Vec<DnsARecord>, ApiError> {
        let records = self
            .perform_dns_lookup(zone_id, &[("type".to_string(), "A".to_string())])
            .await?;

        let a_records: Vec<DnsARecord> = records
            .into_iter()
            .map(|record| -> Result<DnsARecord, std::net::AddrParseError> {
                Ok(DnsARecord {
                    ip: record.content.parse()?,
                    name: record.name,
                    zone_id: record.zone_id,
                    zone_name: record.zone_name,
                    id: record.id,
                    comment: record.comment,
                })
            })
            .filter_map(|result| result.ok())
            .collect();

        return Ok(a_records);
    }
}

impl crate::CloudflareClient {
    async fn perform_dns_lookup(
        &self,
        zone_id: &str,
        query_params: &[(String, String)],
    ) -> Result<DnsRecords, ApiError> {
        let url = Self::DNS_RECORDS_FOR_ZONE_URL.format(&[zone_id]);
        let response = self.client.get(&url).query(query_params).send().await?;
        let mut records: HashMap<String, serde_json::Value> = response.json().await?;

        let dns_record: DnsRecords = serde_json::from_value(
            records
                .remove("result")
                .ok_or_else(|| ApiError::InvalidResponse)?,
        )?;
        return Ok(dns_record);
    }
}
