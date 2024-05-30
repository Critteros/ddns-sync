#![allow(async_fn_in_trait)]
mod api;
pub mod error;
pub mod prelude;

pub use api::client::CloudflareClient;
