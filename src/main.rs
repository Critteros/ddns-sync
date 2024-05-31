#![allow(async_fn_in_trait)]
mod config;
mod ip_lookup;
mod utils;

use std::net::Ipv4Addr;
use std::process::exit;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::Arc;
use std::sync::Mutex;

use anyhow::Result;
use cloudflare_adapter::prelude::*;
use cloudflare_adapter::synchronize_dns_records;
use cloudflare_adapter::CloudflareClient;
use log::info;
use std::time::Duration;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::ip_lookup::lookup_public_ip;

async fn ddns_synchronize(client: &CloudflareClient) -> Result<()> {
    log::info!("Checking public IP");
    static LAST_IP: Mutex<Option<Ipv4Addr>> = Mutex::new(None);

    let public_ip = lookup_public_ip().await?;
    #[allow(unused_assignments)]
    let mut ip_changed = false;

    if let Ok(mut last_ip_mutex) = LAST_IP.lock() {
        let last_ip = last_ip_mutex.as_ref();
        ip_changed = match last_ip {
            Some(last_ip) => last_ip != &public_ip,
            None => true,
        };
        if ip_changed {
            log::info!("IP changed from {:?} to {:?}", last_ip, public_ip);
            *last_ip_mutex = Some(public_ip);
        } else {
            log::info!("IP has not changed");
        }
    } else {
        log::error!("Failed to acquire lock on last IP");
        return Ok(());
    }

    if ip_changed {
        log::info!("Synchronizing DNS records");
        synchronize_dns_records(&client, public_ip).await?;
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let running = Arc::new(AtomicBool::new(true));

    info!("Starting DNS sychronizer");
    let config = config::get_config();
    log::info!("Configuration: {:#?}", config);

    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .expect("Error setting Ctrl-C handler");

    let sched = JobScheduler::new().await.unwrap();
    sched.shutdown_on_ctrl_c();

    let cloudflare_client = CloudflareClient::new(config.cloudflare_api_key.clone());

    log::info!("Verifying token");
    let is_token_valid = cloudflare_client.verify_token().await;

    match is_token_valid {
        Ok(_) => {
            log::info!("Token is valid");
        }
        Err(e) => {
            log::error!("Token is invalid: {}", e);
            exit(-1)
        }
    }

    let job = Job::new_repeated_async(Duration::from_secs(5), move |_uuid, _l| {
        let client = cloudflare_client.clone();
        Box::pin(async move {
            ddns_synchronize(&client).await.unwrap();
        })
    })
    .unwrap();

    sched.add(job).await.unwrap();
    sched.start().await.unwrap();

    while running.load(Ordering::SeqCst) {
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
}
