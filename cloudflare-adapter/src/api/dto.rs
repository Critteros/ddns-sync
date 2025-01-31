use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct ZoneInfo {
    pub id: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct DnsEntry {
    pub id: String,
    pub name: String,
    pub content: String,
    pub comment: Option<String>,
}
