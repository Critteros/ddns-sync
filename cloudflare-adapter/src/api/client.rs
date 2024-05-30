use reqwest::header::{HeaderMap, HeaderValue, AUTHORIZATION};
use reqwest::Client;

#[derive(Debug)]
pub struct CloudflareClient {
    pub(crate) client: Client,
}

impl CloudflareClient {
    pub fn new(api_key: String) -> Self {
        Self {
            client: Self::create_client(&api_key),
        }
    }

    fn create_client(api_token: &str) -> Client {
        let mut headers = HeaderMap::new();
        let mut auth_header_value =
            HeaderValue::from_str(format!("Bearer {}", api_token).as_str()).unwrap();
        auth_header_value.set_sensitive(true);
        headers.insert(AUTHORIZATION, auth_header_value);

        Client::builder().default_headers(headers).build().unwrap()
    }
}
