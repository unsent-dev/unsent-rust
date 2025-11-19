use crate::client::{Client, Result};
use crate::types::*;

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

    pub fn verify(&self, domain_id: i32) -> Result<DomainVerifyResponse> {
        let empty: std::collections::HashMap<String, String> = std::collections::HashMap::new();
        self.client.put(&format!("/domains/{}/verify", domain_id), &empty)
    }

    pub fn get(&self, domain_id: i32) -> Result<Domain> {
        self.client.get(&format!("/domains/{}", domain_id))
    }

    pub fn delete(&self, domain_id: i32) -> Result<DomainDeleteResponse> {
        self.client.delete(&format!("/domains/{}", domain_id))
    }
}
