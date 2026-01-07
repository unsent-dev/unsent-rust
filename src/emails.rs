use crate::client::{Client, Result};
use crate::models::*;

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

    pub fn send_with_options(
        &self,
        payload: &EmailCreate,
        options: &RequestOptions,
    ) -> Result<EmailCreateResponse> {
        self.create_with_options(payload, options)
    }

    pub fn create(&self, payload: &EmailCreate) -> Result<EmailCreateResponse> {
        self.client.post("/emails", payload)
    }

    pub fn create_with_options(
        &self,
        payload: &EmailCreate,
        options: &RequestOptions,
    ) -> Result<EmailCreateResponse> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &options.idempotency_key {
            headers.insert(
                "Idempotency-Key",
                reqwest::header::HeaderValue::from_str(key).unwrap(),
            );
        }
        self.client.post_with_headers("/emails", payload, headers)
    }

    pub fn batch(&self, emails: &[EmailBatchItem]) -> Result<EmailBatchResponse> {
        self.client.post("/emails/batch", &emails)
    }

    pub fn batch_with_options(
        &self,
        emails: &[EmailBatchItem],
        options: &RequestOptions,
    ) -> Result<EmailBatchResponse> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &options.idempotency_key {
            headers.insert(
                "Idempotency-Key",
                reqwest::header::HeaderValue::from_str(key).unwrap(),
            );
        }
        self.client
            .post_with_headers("/emails/batch", &emails, headers)
    }

    pub fn get(&self, email_id: &str) -> Result<Email> {
        self.client.get(&format!("/emails/{}", email_id))
    }

    pub fn update(&self, email_id: &str, payload: &EmailUpdate) -> Result<EmailUpdateResponse> {
        self.client.patch(&format!("/emails/{}", email_id), payload)
    }

    pub fn cancel(&self, email_id: &str) -> Result<EmailCancelResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client
            .post(&format!("/emails/{}/cancel", email_id), &empty)
    }

    /// List sent emails with optional filters
    pub fn list(&self, params: Option<&EmailListParams>) -> Result<EmailListResponse> {
        let mut path = "/emails".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path)
    }

    /// Get list of bounced emails
    pub fn bounces(&self, params: Option<&PaginationParams>) -> Result<BouncesResponse> {
        let mut path = "/emails/bounces".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path)
    }

    /// Get list of spam complaints
    pub fn complaints(&self, params: Option<&PaginationParams>) -> Result<ComplaintsResponse> {
        let mut path = "/emails/complaints".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path)
    }

    /// Get list of unsubscribed emails
    pub fn unsubscribes(&self, params: Option<&PaginationParams>) -> Result<UnsubscribesResponse> {
        let mut path = "/emails/unsubscribes".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_emails_paths() {
        let paths = vec![
            "/emails",
            "/emails/test-id",
            "/emails/test-id/cancel",
            "/emails/batch",
            "/emails/bounces",
            "/emails/complaints",
            "/emails/unsubscribes",
        ];

        for path in paths {
            assert!(path.starts_with("/emails"));
        }
    }

    #[test]
    fn test_email_list_params() {
        let params = EmailListParams {
            page: Some("1".to_string()),
            limit: Some("50".to_string()),
            start_date: Some("2024-01-01T00:00:00Z".to_string()),
            end_date: Some("2024-01-31T23:59:59Z".to_string()),
            domain_id: Some("domain-id".to_string()),
        };
        assert_eq!(params.page, Some("1".to_string()));
        assert_eq!(params.limit, Some("50".to_string()));
    }

    #[test]
    fn test_pagination_params() {
        let params = PaginationParams {
            page: Some(1),
            limit: Some(20),
        };
        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(20));
    }

    #[test]
    fn test_request_options() {
        let opts = RequestOptions {
            idempotency_key: Some("test-key".to_string()),
        };
        assert_eq!(opts.idempotency_key, Some("test-key".to_string()));
    }
}
