use std::time::Duration;

use tokio::time;

mod state;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let mut interval = time::interval(Duration::from_secs(3));

    loop {
        interval.tick().await;
        tokio::spawn(async { println!("Hello World"); });
    }
}
