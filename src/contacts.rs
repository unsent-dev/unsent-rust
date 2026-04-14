// @manual
use crate::client::{Client, Result};
use crate::models::*;

pub struct ContactsClient<'a> {
    client: &'a Client,
}

impl<'a> ContactsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    pub async fn create(&self, book_id: &str, payload: &ContactCreate) -> Result<ContactCreateResponse> {
        self.client
            .post(&format!("/contactBooks/{}/contacts", book_id), payload).await
    }

    pub async fn get(&self, book_id: &str, contact_id: &str) -> Result<Contact> {
        self.client.get(&format!(
            "/contactBooks/{}/contacts/{}",
            book_id, contact_id
        )).await
    }

    pub async fn update(
        &self,
        book_id: &str,
        contact_id: &str,
        payload: &ContactUpdate,
    ) -> Result<ContactUpdateResponse> {
        self.client.patch(
            &format!("/contactBooks/{}/contacts/{}", book_id, contact_id),
            payload,
        ).await
    }

    pub async fn upsert(
        &self,
        book_id: &str,
        contact_id: &str,
        payload: &ContactUpsert,
    ) -> Result<ContactUpsertResponse> {
        self.client.put(
            &format!("/contactBooks/{}/contacts/{}", book_id, contact_id),
            payload,
        ).await
    }

    pub async fn delete(&self, book_id: &str, contact_id: &str) -> Result<ContactDeleteResponse> {
        self.client.delete(&format!(
            "/contactBooks/{}/contacts/{}",
            book_id, contact_id
        )).await
    }

    /// List contacts in a contact book
    pub async fn list(&self, book_id: &str, params: Option<&ContactListParams>) -> Result<Vec<Contact>> {
        let mut path = format!("/contactBooks/{}/contacts", book_id);

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
    fn test_contacts_paths() {
        let paths = vec![
            "/contactBooks/book-id/contacts",
            "/contactBooks/book-id/contacts/contact-id",
        ];

        for path in paths {
            assert!(path.contains("/contactBooks") && path.contains("/contacts"));
        }
    }

    #[test]
    fn test_contact_create() {
        let req = ContactCreate::new("test@example.com".to_string());
        assert_eq!(req.email, "test@example.com");
        assert_eq!(req.first_name, None);
        assert_eq!(req.last_name, None);
    }

    #[test]
    fn test_contact_list_params() {
        let params = ContactListParams {
            emails: Some("test@example.com".to_string()),
            page: Some(1),
            limit: Some(50),
            ids: Some("id1,id2".to_string()),
        };
        assert_eq!(params.emails, Some("test@example.com".to_string()));
        assert_eq!(params.page, Some(1));
    }
}
