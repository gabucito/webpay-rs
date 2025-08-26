use reqwest::header::{HeaderMap, HeaderValue, CONTENT_TYPE, ACCEPT};
use reqwest::Client as HttpClient;
use std::time::Duration;

/// The Transbank environment to use.
#[derive(Clone, Debug)]
pub enum Environment {
    /// Integration environment for testing.
    Integration,
    /// Production environment for real transactions.
    Production,
}

impl Environment {
    /// Returns the base URL for the environment.
    pub fn base_url(&self) -> &'static str {
        match self {
            Environment::Integration => "https://webpay3gint.transbank.cl",
            Environment::Production => "https://webpay3g.transbank.cl",
        }
    }
}

/// The credentials for the Webpay API.
#[derive(Clone, Debug)]
pub struct Credentials {
    /// `Tbk-Api-Key-Id` — usually your commerce code (e.g., 597055555532 for Webpay Plus integration)
    pub commerce_code: String,
    /// `Tbk-Api-Key-Secret` — API key secret provided by Transbank
    pub api_key: String,
}

/// The Webpay client.
#[derive(Clone)]
pub struct WebpayClient {
    pub env: Environment,
    pub creds: Credentials,
    http: HttpClient,
}

impl WebpayClient {
    /// Creates a new Webpay client.
    pub fn new(env: Environment, creds: Credentials) -> Self {
        Self::new_with_timeout(env, creds, Duration::from_secs(20))
    }

    /// Creates a new Webpay client with a custom timeout.
    pub fn new_with_timeout(env: Environment, creds: Credentials, timeout: Duration) -> Self {
        let http = HttpClient::builder()
            .timeout(timeout)
            .build()
            .expect("reqwest client");
        Self { env, creds, http }
    }

    fn headers(&self) -> HeaderMap {
        let mut h = HeaderMap::new();
        h.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        h.insert(ACCEPT, HeaderValue::from_static("application/json"));
        h.insert("Tbk-Api-Key-Id", HeaderValue::from_str(&self.creds.commerce_code).unwrap());
        h.insert("Tbk-Api-Key-Secret", HeaderValue::from_str(&self.creds.api_key).unwrap());
        h
    }

    /// Returns a reference to the underlying HTTP client.
    pub fn http(&self) -> &HttpClient { &self.http }

    /// Returns the full URL for a given path.
    pub fn endpoint(&self, path: &str) -> String {
        format!("{}{}", self.env.base_url(), path)
    }

    /// Returns the headers for a request.
    pub fn headers_ref(&self) -> HeaderMap { self.headers() }
}
