use std::fmt;
use std::sync::OnceLock;

use clap::Parser;

use crate::utils::mask_display;

#[derive(Parser)]
#[command(version, author, about)]
pub struct Config {
    #[arg(long, env("CLOUDFLARE_API_KEY"))]
    pub cloudflare_api_key: String,
}

impl fmt::Debug for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Config {{ cloudflare_api_key: \"{}\" }}",
            mask_display(&self.cloudflare_api_key)
        )
    }
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
