#![allow(async_fn_in_trait)]
mod config;
mod ip_lookup;
mod utils;

use cloudflare_adapter::prelude::*;
use cloudflare_adapter::CloudflareClient;

use crate::ip_lookup::lookup_public_ip;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let config = config::get_config();
    println!("Config: {:?}", config);
    let cloudflare_client = CloudflareClient::new(config.cloudflare_api_key.clone());
    let is_token_valid = cloudflare_client.verify_token().await.unwrap();
    println!("Token is valid: {}", is_token_valid);
    let zones = cloudflare_client.list_zones().await.unwrap();
    println!("Zones: {:?}", zones);

    let main_zone = &zones[0].id;

    let public_ip = lookup_public_ip().await.unwrap();
    println!("Public IP: {}", public_ip);

    let dns_records = cloudflare_client
        .dns_records_for_zone(main_zone)
        .await
        .unwrap();
    println!("DNS Records: {:?}", dns_records);
}
