use std::net::Ipv4Addr;

use anyhow::Result;
use reqwest::{get, Url};
use serde::Deserialize;

const IPV6_LOOKUP_API: &'static str = "https://api6.ipify.org";

pub async fn ipv4_lookup() -> Result<Ipv4Addr> {
    const IPV4_LOOKUP_API: &'static str = "https://api.ipify.org";
    #[derive(Debug, Deserialize)]
    struct IpApiResponse {
        ip: Ipv4Addr,
    }

    let query_params = [
        ("format", "json")
    ];
    let url = Url::parse_with_params(IPV4_LOOKUP_API, query_params)?;
    let res = get(url).await?.error_for_status()?;
    Ok(res.json::<IpApiResponse>().await?.ip)
}