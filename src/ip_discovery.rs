use std::net::{Ipv4Addr, Ipv6Addr};

use anyhow::Result;
use reqwest::{get, Url};
use serde::Deserialize;

pub struct IpLookupApi<'a> {
    ipv4_lookup_url: &'a str,
    ipv6_lookup_url: &'a str,
}

impl<'a> Default for IpLookupApi<'a> {
    fn default() -> Self {
        const IPV6_LOOKUP_API: &'static str = "https://api6.ipify.org";
        const IPV4_LOOKUP_API: &'static str = "https://api.ipify.org";

        IpLookupApi {
            ipv6_lookup_url: &IPV6_LOOKUP_API,
            ipv4_lookup_url: &IPV4_LOOKUP_API,
        }
    }
}

pub trait Ipv4Resolver {
    async fn ipv4_lookup(&self) -> Result<Ipv4Addr>;
}

pub trait Ipv6Resolver {
    async fn ipv6_lookup(&self) -> Result<Ipv6Addr>;
}

impl<'a> Ipv4Resolver for IpLookupApi<'a> {
    async fn ipv4_lookup(&self) -> Result<Ipv4Addr> {
        #[derive(Debug, Deserialize)]
        struct IpApiResponse {
            ip: Ipv4Addr,
        }

        let query_params = [
            ("format", "json")
        ];
        let url = Url::parse_with_params(self.ipv4_lookup_url, query_params)?;
        let res = get(url).await?.error_for_status()?;
        Ok(res.json::<IpApiResponse>().await?.ip)
    }
}

impl<'a> Ipv6Resolver for IpLookupApi<'a> {
    async fn ipv6_lookup(&self) -> Result<Ipv6Addr> {
        #[derive(Debug, Deserialize)]
        struct IpApiResponse {
            ip: Ipv6Addr,
        }

        let query_params = [
            ("format", "json")
        ];
        let url = Url::parse_with_params(self.ipv6_lookup_url, query_params)?;
        let res = get(url).await?.error_for_status()?;
        Ok(res.json::<IpApiResponse>().await?.ip)
    }
}

#[cfg(test)]
mod tests {
    use mockito::{Matcher, Server};
    use serde_json::json;

    use super::*;

    #[tokio::test]
    async fn test_lookup_ipv4_success() {
        let mut server = Server::new_async().await;
        server.mock("GET", "/").with_body(json!({"ip": "10.0.0.1"}).to_string())
            .match_query(Matcher::UrlEncoded("format".into(), "json".into()))
            .create_async().await;

        let url = server.url();
        let ip = IpLookupApi { ipv4_lookup_url: url.as_str(), ipv6_lookup_url: url.as_str() }.ipv4_lookup().await.unwrap();
        assert_eq!(ip, "10.0.0.1".parse::<Ipv4Addr>().unwrap());
    }

    #[tokio::test]
    async fn test_lookup_ipv6_success() {
        let mut server = Server::new_async().await;
        server.mock("GET", "/").with_body(json!({"ip":"2a00:1450:400f:80d::200e"}).to_string())
            .match_query(Matcher::UrlEncoded("format".into(), "json".into()))
            .create_async().await;

        let url = server.url();
        let ip = IpLookupApi { ipv6_lookup_url: url.as_str(), ipv4_lookup_url: url.as_str() }.ipv6_lookup().await.unwrap();
        assert_eq!(ip, "2a00:1450:400f:80d::200e".parse::<Ipv6Addr>().unwrap());
    }
}