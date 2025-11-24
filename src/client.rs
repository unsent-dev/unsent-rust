use crate::types::*;
use reqwest::blocking::Client as HttpClient;
use std::env;

const DEFAULT_BASE_URL: &str = "https://api.unsent.dev";

#[derive(thiserror::Error, Debug)]
pub enum UnsentError {
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("API error: {0}")]
    Api(#[from] APIError),
    
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
    
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, UnsentError>;

pub struct Client {
    key: String,
    base_url: String,
    http_client: HttpClient,
    raise_on_error: bool,
}

impl Client {
    pub fn new(key: impl Into<String>) -> Result<Self> {
        let key = key.into();
        let key = if key.is_empty() {
            env::var("UNSENT_API_KEY")
                .or_else(|_| env::var("UNSENT_API_KEY"))
                .map_err(|_| UnsentError::Other("Missing API key. Pass it to Client::new or set UNSENT_API_KEY environment variable".to_string()))?
        } else {
            key
        };

        let base_url = env::var("UNSENT_BASE_URL")
            .or_else(|_| env::var("UNSENT_BASE_URL"))
            .unwrap_or_else(|_| DEFAULT_BASE_URL.to_string());

        Ok(Self {
            key,
            base_url: format!("{}/v1", base_url),
            http_client: HttpClient::new(),
            raise_on_error: true,
        })
    }

    pub fn with_base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = format!("{}/v1", url.into());
        self
    }

    pub fn with_http_client(mut self, client: HttpClient) -> Self {
        self.http_client = client;
        self
    }

    pub fn with_raise_on_error(mut self, raise: bool) -> Self {
        self.raise_on_error = raise;
        self
    }

    fn request<T: serde::de::DeserializeOwned>(
        &self,
        method: reqwest::Method,
        path: &str,
        body: Option<&impl serde::Serialize>,
        headers: Option<reqwest::header::HeaderMap>,
    ) -> Result<T> {
        let url = format!("{}{}", self.base_url, path);
        
        let mut request = self.http_client
            .request(method.clone(), &url)
            .header("Authorization", format!("Bearer {}", self.key))
            .header("Content-Type", "application/json");

        if let Some(h) = headers {
            request = request.headers(h);
        }

        if let Some(body) = body {
            request = request.json(body);
        }

        let response = request.send()?;
        let status = response.status();
        let body_text = response.text()?;
        
        if !status.is_success() {
            let api_error: APIError = serde_json::from_str(&body_text).unwrap_or_else(|_| APIError {
                code: "INTERNAL_SERVER_ERROR".to_string(),
                message: "Unknown error".to_string(),
            });
            
            if self.raise_on_error {
                return Err(UnsentError::Api(api_error));
            }
        }

        Ok(serde_json::from_str(&body_text)?)
    }

    pub fn post<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T> {
        self.request(reqwest::Method::POST, path, Some(body), None)
    }

    pub fn post_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
        headers: reqwest::header::HeaderMap,
    ) -> Result<T> {
        self.request(reqwest::Method::POST, path, Some(body), Some(headers))
    }

    pub fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request::<T>(reqwest::Method::GET, path, None::<&()>, None)
    }

    pub fn get_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        headers: reqwest::header::HeaderMap,
    ) -> Result<T> {
        self.request::<T>(reqwest::Method::GET, path, None::<&()>, Some(headers))
    }

    pub fn put<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T> {
        self.request(reqwest::Method::PUT, path, Some(body), None)
    }

    pub fn put_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
        headers: reqwest::header::HeaderMap,
    ) -> Result<T> {
        self.request(reqwest::Method::PUT, path, Some(body), Some(headers))
    }

    pub fn patch<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
    ) -> Result<T> {
        self.request(reqwest::Method::PATCH, path, Some(body), None)
    }

    pub fn patch_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        body: &impl serde::Serialize,
        headers: reqwest::header::HeaderMap,
    ) -> Result<T> {
        self.request(reqwest::Method::PATCH, path, Some(body), Some(headers))
    }

    pub fn delete<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        self.request::<T>(reqwest::Method::DELETE, path, None::<&()>, None)
    }

    pub fn delete_with_headers<T: serde::de::DeserializeOwned>(
        &self,
        path: &str,
        headers: reqwest::header::HeaderMap,
    ) -> Result<T> {
        self.request::<T>(reqwest::Method::DELETE, path, None::<&()>, Some(headers))
    }
}
