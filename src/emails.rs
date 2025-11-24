use crate::client::{Client, Result};
use crate::types::*;

pub struct EmailsClient<'a> {
    client: &'a Client,
}

impl<'a> EmailsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn send(&self, payload: &EmailCreate) -> Result<EmailCreateResponse> {
        self.create(payload)
    }

    pub fn send_with_options(&self, payload: &EmailCreate, options: &RequestOptions) -> Result<EmailCreateResponse> {
        self.create_with_options(payload, options)
    }

    pub fn create(&self, payload: &EmailCreate) -> Result<EmailCreateResponse> {
        self.client.post("/emails", payload)
    }

    pub fn create_with_options(&self, payload: &EmailCreate, options: &RequestOptions) -> Result<EmailCreateResponse> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &options.idempotency_key {
            headers.insert("Idempotency-Key", reqwest::header::HeaderValue::from_str(key).unwrap());
        }
        self.client.post_with_headers("/emails", payload, headers)
    }

    pub fn batch(&self, emails: &[EmailBatchItem]) -> Result<EmailBatchResponse> {
        self.client.post("/emails/batch", &emails)
    }

    pub fn batch_with_options(&self, emails: &[EmailBatchItem], options: &RequestOptions) -> Result<EmailBatchResponse> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &options.idempotency_key {
            headers.insert("Idempotency-Key", reqwest::header::HeaderValue::from_str(key).unwrap());
        }
        self.client.post_with_headers("/emails/batch", &emails, headers)
    }

    pub fn get(&self, email_id: &str) -> Result<Email> {
        self.client.get(&format!("/emails/{}", email_id))
    }

    pub fn update(&self, email_id: &str, payload: &EmailUpdate) -> Result<EmailUpdateResponse> {
        self.client.patch(&format!("/emails/{}", email_id), payload)
    }

    pub fn cancel(&self, email_id: &str) -> Result<EmailCancelResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client.post(&format!("/emails/{}/cancel", email_id), &empty)
    }
}
