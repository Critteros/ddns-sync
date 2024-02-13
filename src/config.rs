use std::sync::OnceLock;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Config {
    #[arg(long, env("CLOUDFLARE_API_KEY"))]
    cloudflare_api_key: String
}

impl Config {
    fn new() -> Self {
        dotenv::dotenv().ok();
        Self::parse()
    }
}

pub fn get_config() -> &'static Config {
    static CONFIG: OnceLock<Config> = OnceLock::new();
    CONFIG.get_or_init(|| Config::new())
}

