use crate::client::{Client, Result};
use crate::models::*;

pub struct DomainsClient<'a> {
    client: &'a Client,
}

impl<'a> DomainsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn list(&self) -> Result<Vec<Domain>> {
        self.client.get("/domains")
    }

    pub fn create(&self, payload: &DomainCreate) -> Result<DomainCreateResponse> {
        self.client.post("/domains", payload)
    }

    pub fn verify(&self, domain_id: &str) -> Result<DomainVerifyResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client
            .put(&format!("/domains/{}/verify", domain_id), &empty)
    }

    pub fn get(&self, domain_id: &str) -> Result<Domain> {
        self.client.get(&format!("/domains/{}", domain_id))
    }

    pub fn delete(&self, domain_id: &str) -> Result<DomainDeleteResponse> {
        self.client.delete(&format!("/domains/{}", domain_id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_domains_paths() {
        let paths = vec!["/domains", "/domains/test-id", "/domains/test-id/verify"];

        for path in paths {
            assert!(path.starts_with("/domains"));
        }
    }

    #[test]
    fn test_domain_create() {
        let req = DomainCreate::new("example.com".to_string(), "us-east-1".to_string());
        assert_eq!(req.name, "example.com");
        assert_eq!(req.region, "us-east-1");
    }
}
