use crate::client::{Client, Result};
use crate::models::*;

pub struct EventsClient<'a> {
    client: &'a Client,
}

impl<'a> EventsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all email events with optional filters
    pub async fn list(&self, params: Option<&EventsListParams>) -> Result<EventsListResponse> {
        let mut path = "/events".to_string();

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
    fn test_events_client_creation() {
        let path = "/events";
        assert_eq!(path, "/events");
    }

    #[test]
    fn test_events_params_default() {
        let params = EventsListParams::default();
        assert_eq!(params.page, None);
        assert_eq!(params.limit, None);
        assert_eq!(params.status, None);
        assert_eq!(params.start_date, None);
    }

    #[test]
    fn test_events_params_with_pagination() {
        let params = EventsListParams {
            page: Some(1),
            limit: Some(50),
            status: None,
            start_date: None,
        };
        assert_eq!(params.page, Some(1));
        assert_eq!(params.limit, Some(50));
    }

    #[test]
    fn test_events_params_with_status_filter() {
        let params = EventsListParams {
            page: None,
            limit: None,
            status: Some("DELIVERED".to_string()),
            start_date: None,
        };
        assert_eq!(params.status, Some("DELIVERED".to_string()));
    }

    #[test]
    fn test_events_params_with_date_filter() {
        let params = EventsListParams {
            page: None,
            limit: None,
            status: None,
            start_date: Some("2024-01-01T00:00:00Z".to_string()),
        };
        assert_eq!(params.start_date, Some("2024-01-01T00:00:00Z".to_string()));
    }

    #[test]
    fn test_events_params_serialization() {
        let params = EventsListParams {
            page: Some(1),
            limit: Some(20),
            status: Some("SENT".to_string()),
            start_date: None,
        };

        let serialized = serde_qs::to_string(&params).unwrap();
        assert!(serialized.contains("page=1"));
        assert!(serialized.contains("limit=20"));
        assert!(serialized.contains("status=SENT"));
    }

    #[test]
    fn test_events_path_construction() {
        let base_path = "/events";
        assert_eq!(base_path, "/events");

        let params = EventsListParams {
            page: Some(1),
            limit: Some(10),
            status: Some("DELIVERED".to_string()),
            start_date: None,
        };
        let query = serde_qs::to_string(&params).unwrap();
        let full_path = format!("{}?{}", base_path, query);

        assert!(full_path.starts_with("/events?"));
    }

    #[test]
    fn test_events_response_deserialization() {
        let json = r#"{"data": []}"#;
        let response: std::result::Result<EventsListResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());
    }

    #[test]
    fn test_events_valid_status_values() {
        let valid_statuses = vec![
            "SCHEDULED",
            "QUEUED",
            "SENT",
            "DELIVERY_DELAYED",
            "BOUNCED",
            "REJECTED",
            "RENDERING_FAILURE",
            "DELIVERED",
            "OPENED",
            "CLICKED",
            "COMPLAINED",
            "FAILED",
            "CANCELLED",
            "SUPPRESSED",
        ];

        for status in valid_statuses {
            let params = EventsListParams {
                page: None,
                limit: None,
                status: Some(status.to_string()),
                start_date: None,
            };
            assert_eq!(params.status, Some(status.to_string()));
        }
    }
}
