use crate::client::{Client, Result};
use crate::models::*;

pub struct TemplatesClient<'a> {
    client: &'a Client,
}

impl<'a> TemplatesClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all templates
    pub fn list(&self) -> Result<Vec<Template>> {
        self.client.get("/templates")
    }

    /// Create a new template
    pub fn create(&self, payload: &TemplateCreate) -> Result<TemplateCreateResponse> {
        self.client.post("/templates", payload)
    }

    /// Get template details by ID
    pub fn get(&self, id: &str) -> Result<Template> {
        self.client.get(&format!("/templates/{}", id))
    }

    /// Update a template
    pub fn update(&self, id: &str, payload: &TemplateUpdate) -> Result<TemplateUpdateResponse> {
        self.client.patch(&format!("/templates/{}", id), payload)
    }

    /// Delete a template
    pub fn delete(&self, id: &str) -> Result<TemplateDeleteResponse> {
        self.client.delete(&format!("/templates/{}", id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_templates_paths() {
        let paths = vec!["/templates", "/templates/test-id"];

        for path in paths {
            assert!(path.starts_with("/templates"));
        }
    }

    #[test]
    fn test_template_create() {
        let req = TemplateCreate::new(
            "Welcome Email".to_string(),
            "Welcome to Our Service".to_string(),
        );
        assert_eq!(req.name, "Welcome Email");
        assert_eq!(req.subject, "Welcome to Our Service");
        assert_eq!(req.html, None);
        assert_eq!(req.content, None);
    }

    #[test]
    fn test_template_create_with_content() {
        let req = TemplateCreate {
            name: "Welcome Email".to_string(),
            subject: "Welcome {{name}}".to_string(),
            html: Some("<h1>Welcome {{name}}</h1>".to_string()),
            content: Some("Welcome {{name}}".to_string()),
        };
        assert_eq!(req.html, Some("<h1>Welcome {{name}}</h1>".to_string()));
        assert_eq!(req.content, Some("Welcome {{name}}".to_string()));
    }

    #[test]
    fn test_template_update() {
        let req = TemplateUpdate {
            name: Some("Updated Template".to_string()),
            subject: Some("New Subject".to_string()),
            html: Some("<p>New content</p>".to_string()),
            content: None,
        };
        assert_eq!(req.name, Some("Updated Template".to_string()));
        assert_eq!(req.subject, Some("New Subject".to_string()));
    }
}
