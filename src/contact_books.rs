// @manual
use crate::client::{Client, Result};
use crate::models::*;

pub struct ContactBooksClient<'a> {
    client: &'a Client,
}

impl<'a> ContactBooksClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all contact books
    pub async fn list(&self) -> Result<Vec<ContactBook>> {
        self.client.get("/contactBooks").await
    }

    /// Create a new contact book
    pub async fn create(&self, payload: &ContactBookCreate) -> Result<ContactBook> {
        self.client.post("/contactBooks", payload).await
    }

    /// Get contact book details by ID
    pub async fn get(&self, id: &str) -> Result<serde_json::Value> {
        self.client.get(&format!("/contactBooks/{}", id)).await
    }

    /// Update a contact book
    pub async fn update(
        &self,
        id: &str,
        payload: &ContactBookUpdate,
    ) -> Result<ContactBookUpdateResponse> {
        self.client.patch(&format!("/contactBooks/{}", id), payload).await
    }

    /// Delete a contact book
    pub async fn delete(&self, id: &str) -> Result<ContactBookDeleteResponse> {
        self.client.delete(&format!("/contactBooks/{}", id)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_contact_books_paths() {
        let paths = vec!["/contactBooks", "/contactBooks/test-id"];

        for path in paths {
            assert!(path.starts_with("/contactBooks"));
        }
    }

    #[test]
    fn test_contact_book_create() {
        let req = ContactBookCreate::new("Test Book".to_string());
        assert_eq!(req.name, "Test Book");
        assert_eq!(req.emoji, None);
    }

    #[test]
    fn test_contact_book_create_with_emoji() {
        let req = ContactBookCreate {
            name: "Test Book".to_string(),
            emoji: Some("📧".to_string()),
            properties: None,
        };
        assert_eq!(req.name, "Test Book");
        assert_eq!(req.emoji, Some("📧".to_string()));
    }

    #[test]
    fn test_contact_book_update() {
        let req = ContactBookUpdate {
            name: Some("Updated Book".to_string()),
            emoji: Some("📬".to_string()),
            properties: None,
        };
        assert_eq!(req.name, Some("Updated Book".to_string()));
        assert_eq!(req.emoji, Some("📬".to_string()));
    }
}
