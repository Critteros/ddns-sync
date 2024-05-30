use std::net::Ipv4Addr;

use thiserror::Error;

pub const IPV4_PRIMARY_LOOKUP_URL: &'static str = "https://api.ipify.org";
pub const IPV4_SECONDARY_URL: &'static str = "https://ipv4.icanhazip.com";

#[derive(Error, Debug)]
pub enum IpLookupError {
    #[error(transparent)]
    NetworkError(#[from] reqwest::Error),

    #[error("Service unavailable")]
    ServiceUnavailable,

    #[error(transparent)]
    IpParseError(#[from] std::net::AddrParseError),
}

pub async fn lookup_public_ip_from_service(url: &str) -> Result<Ipv4Addr, IpLookupError> {
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(IpLookupError::ServiceUnavailable);
    }

    let ip_text = response.text().await?;
    let ip: Ipv4Addr = ip_text.trim().parse()?;
    Ok(ip)
}

pub async fn lookup_public_ip() -> Result<Ipv4Addr, IpLookupError> {
    let primary_ip = lookup_public_ip_from_service(IPV4_PRIMARY_LOOKUP_URL).await;
    match primary_ip {
        Ok(ip) => Ok(ip),
        Err(_) => lookup_public_ip_from_service(IPV4_SECONDARY_URL).await,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::Server;
    use rstest::*;

    #[rstest]
    #[tokio::test]
    async fn ipv4_lookup_success() {
        let mut server = Server::new_async().await;
        server
            .mock("GET", "/")
            .with_body("10.0.0.1")
            .create_async()
            .await;
        let url = server.url();
        let ip = lookup_public_ip_from_service(&url).await.unwrap();
        assert_eq!(ip, "10.0.0.1".parse::<Ipv4Addr>().unwrap())
    }

    #[rstest]
    #[tokio::test]
    async fn ipv4_lookup_failure() {
        let mut server = Server::new_async().await;
        server
            .mock("GET", "/")
            .with_status(500)
            .create_async()
            .await;
        let url = server.url();
        let ip = lookup_public_ip_from_service(&url).await;
        assert!(ip.is_err());
        let err = ip.unwrap_err();
        assert!(matches!(err, IpLookupError::ServiceUnavailable));
    }

    #[rstest]
    #[tokio::test]
    async fn ipv4_parse_failure() {
        let mut server = Server::new_async().await;
        server
            .mock("GET", "/")
            .with_body("not valid ip")
            .create_async()
            .await;
        let url = server.url();
        let ip = lookup_public_ip_from_service(&url).await;
        assert!(ip.is_err());
        let err = ip.unwrap_err();
        assert!(matches!(err, IpLookupError::IpParseError(_)));
    }
}
