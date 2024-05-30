use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ZoneInfo {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DnsEntry {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub name: String,
    #[serde(rename = "type")]
    pub dns_record_type: String,
    pub content: String,
}
