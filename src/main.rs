use std::time::Duration;

use tokio::time;

mod cloudflare;
mod config;
mod ip_discovery;
mod state;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut interval = time::interval(Duration::from_secs(3));
    println!("{:?}", config::get_config());

    let api = cloudflare::CloudflareApi::from_config(config::get_config());
    println!("{:?}", api.check_token().await.unwrap());

    // loop {
    //     interval.tick().await;
    //     // tokio::spawn(async {
    //     //     // let ip = ip_discovery::ipv4_lookup().await.unwrap();
    //     //     println!("Ipv4: {ip}");
    //     // });
    // }
}
