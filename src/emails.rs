use crate::client::{Client, Result};
use crate::models::*;

pub struct EmailsClient<'a> {
    client: &'a Client,
}

impl<'a> EmailsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn send(&self, payload: &EmailCreate) -> Result<EmailCreateResponse> {
        self.create(payload).await
    }

    pub async fn send_with_options(
        &self,
        payload: &EmailCreate,
        options: &RequestOptions,
    ) -> Result<EmailCreateResponse> {
        self.create_with_options(payload, options).await
    }

    pub async fn create(&self, payload: &EmailCreate) -> Result<EmailCreateResponse> {
        self.client.post("/emails", payload).await
    }

    pub async fn create_with_options(
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
        self.client.post_with_headers("/emails", payload, headers).await
    }

    pub async fn batch(&self, emails: &[EmailBatchItem]) -> Result<serde_json::Value> {
        self.client.post("/emails/batch", &emails).await
    }

    pub async fn batch_with_options(
        &self,
        emails: &[EmailBatchItem],
        options: &RequestOptions,
    ) -> Result<serde_json::Value> {
        let mut headers = reqwest::header::HeaderMap::new();
        if let Some(key) = &options.idempotency_key {
            headers.insert(
                "Idempotency-Key",
                reqwest::header::HeaderValue::from_str(key).unwrap(),
            );
        }
        self.client
            .post_with_headers("/emails/batch", &emails, headers).await
    }

    pub async fn get(&self, email_id: &str) -> Result<Email> {
        self.client.get(&format!("/emails/{}", email_id)).await
    }

    pub async fn update(&self, email_id: &str, payload: &EmailUpdate) -> Result<EmailUpdateResponse> {
        self.client.patch(&format!("/emails/{}", email_id), payload).await
    }

    pub async fn cancel(&self, email_id: &str) -> Result<EmailCancelResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client
            .post(&format!("/emails/{}/cancel", email_id), &empty).await
    }

    /// List sent emails with optional filters
    pub async fn list(&self, params: Option<&EmailListParams>) -> Result<EmailListResponse> {
        let mut path = "/emails".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Get list of bounced emails
    pub async fn bounces(&self, params: Option<&PaginationParams>) -> Result<BouncesResponse> {
        let mut path = "/emails/bounces".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Get list of spam complaints
    pub async fn complaints(&self, params: Option<&PaginationParams>) -> Result<ComplaintsResponse> {
        let mut path = "/emails/complaints".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Get list of unsubscribed emails
    pub async fn unsubscribes(&self, params: Option<&PaginationParams>) -> Result<UnsubscribesResponse> {
        let mut path = "/emails/unsubscribes".to_string();

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
    }

    /// Get events for a specific email
    pub async fn get_events(
        &self,
        email_id: &str,
        params: Option<&EmailEventsParams>,
    ) -> Result<EmailEventsResponse> {
        let mut path = format!("/emails/{}/events", email_id);

        if let Some(p) = params {
            let query_params = serde_qs::to_string(p).unwrap_or_default();
            if !query_params.is_empty() {
                path = format!("{}?{}", path, query_params);
            }
        }

        self.client.get(&path).await
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

    #[test]
    fn test_email_events_path() {
        let email_id = "email-123";
        let path = format!("/emails/{}/events", email_id);
        assert_eq!(path, "/emails/email-123/events");
        assert!(path.contains("/events"));
    }

    #[test]
    fn test_email_events_params_default() {
        let params = EmailEventsParams::default();
        assert_eq!(params.page, None);
        assert_eq!(params.limit, None);
    }

    #[test]
    fn test_email_events_params_with_pagination() {
        let params = EmailEventsParams {
            page: Some(1),
            limit: Some(20),
        };
        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(20));
    }

    #[test]
    fn test_email_events_params_serialization() {
        let params = EmailEventsParams {
            page: Some(2),
            limit: Some(50),
        };

        let serialized = serde_qs::to_string(&params).unwrap();
        assert!(serialized.contains("page=2"));
        assert!(serialized.contains("limit=50"));
    }

    #[test]
    fn test_emails_paths_extended() {
        let paths = vec![
            "/emails",
            "/emails/test-id",
            "/emails/test-id/cancel",
            "/emails/test-id/events",
            "/emails/batch",
            "/emails/bounces",
            "/emails/complaints",
            "/emails/unsubscribes",
        ];

        for path in paths {
            assert!(path.starts_with("/emails"));
        }
    }
}
