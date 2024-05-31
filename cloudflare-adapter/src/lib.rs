#![allow(async_fn_in_trait)]
mod api;
pub mod error;
pub mod prelude;

use std::net::Ipv4Addr;

pub use api::client::CloudflareClient;
use error::ApiError;
use futures::future::join_all;
use prelude::{DnsLookupApi, ZoneInfoApi};

fn starts_with_case_insensitive(string: &str, prefix: &str) -> bool {
    string.to_lowercase().starts_with(&prefix.to_lowercase())
}

const PREFIX: &'static str = "DDNS";

async fn synchronize_dns_records_for_zone(
    client: &CloudflareClient,
    zone_id: &str,
    new_ip: Ipv4Addr,
) -> Result<(), ApiError> {
    let dns_records = client.dns_A_records_for_zone(zone_id).await?;
    let record_ids = dns_records
        .into_iter()
        .filter(|record| {
            record.comment.is_some()
                && starts_with_case_insensitive(record.comment.as_ref().unwrap(), PREFIX)
        })
        .map(|record| record.id)
        .collect::<Vec<_>>();

    let tasks = record_ids
        .iter()
        .map(|record_id| client.update_dns_A_record_ip(zone_id, &record_id, new_ip));
    let results = join_all(tasks).await;
    results.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(())
}

pub async fn synchronize_dns_records(
    client: &CloudflareClient,
    new_ip: Ipv4Addr,
) -> Result<(), ApiError> {
    let zones = client.list_zones().await?;
    let tasks = zones
        .iter()
        .map(|zone| synchronize_dns_records_for_zone(client, &zone.id, new_ip));
    let results = join_all(tasks).await;
    results.into_iter().collect::<Result<Vec<_>, _>>()?;

    Ok(())
}
