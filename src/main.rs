use std::time::Duration;

use tokio::time;

mod state;
mod ip_discovery;
mod config;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut interval = time::interval(Duration::from_secs(3));
    println!("{:?}", config::get_config());

    // loop {
    //     interval.tick().await;
    //     // tokio::spawn(async {
    //     //     // let ip = ip_discovery::ipv4_lookup().await.unwrap();
    //     //     println!("Ipv4: {ip}");
    //     // });
    // }
}
