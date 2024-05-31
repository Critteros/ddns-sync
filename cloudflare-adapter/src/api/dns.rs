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
    const DNS_RECORD_UPDATE_URL: &'static str =
        "https://api.cloudflare.com/client/v4/zones/{}/dns_records/{}";
    #[allow(non_snake_case)]
    async fn dns_A_records_for_zone(&self, zone_id: &str) -> Result<Vec<DnsARecord>, ApiError>;
    #[allow(non_snake_case)]
    async fn update_dns_A_record_ip(
        &self,
        zone_id: &str,
        dns_record_id: &str,
        new_ip: Ipv4Addr,
    ) -> Result<(), ApiError>;
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

    #[allow(non_snake_case)]
    async fn update_dns_A_record_ip(
        &self,
        zone_id: &str,
        dns_record_id: &str,
        new_ip: Ipv4Addr,
    ) -> Result<(), ApiError> {
        let url = Self::DNS_RECORD_UPDATE_URL.format(&[zone_id, dns_record_id]);
        let body = serde_json::json!({
            "type": "A",
            "content": new_ip.to_string(),
        });

        let response = self.client.patch(&url).json(&body).send().await?;
        if response.status().is_success() {
            Ok(())
        } else {
            Err(ApiError::InvalidResponse)
        }
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
