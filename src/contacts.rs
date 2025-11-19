use crate::client::{Client, Result};
use crate::types::*;

pub struct ContactsClient<'a> {
    client: &'a Client,
}

impl<'a> ContactsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub fn create(&self, book_id: &str, payload: &ContactCreate) -> Result<ContactCreateResponse> {
        self.client.post(&format!("/contactBooks/{}/contacts", book_id), payload)
    }

    pub fn get(&self, book_id: &str, contact_id: &str) -> Result<Contact> {
        self.client.get(&format!("/contactBooks/{}/contacts/{}", book_id, contact_id))
    }

    pub fn update(&self, book_id: &str, contact_id: &str, payload: &ContactUpdate) -> Result<ContactUpdateResponse> {
        self.client.patch(&format!("/contactBooks/{}/contacts/{}", book_id, contact_id), payload)
    }

    pub fn upsert(&self, book_id: &str, contact_id: &str, payload: &ContactUpsert) -> Result<ContactUpsertResponse> {
        self.client.put(&format!("/contactBooks/{}/contacts/{}", book_id, contact_id), payload)
    }

    pub fn delete(&self, book_id: &str, contact_id: &str) -> Result<ContactDeleteResponse> {
        self.client.delete(&format!("/contactBooks/{}/contacts/{}", book_id, contact_id))
    }
}
