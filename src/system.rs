use crate::client::{Client, Result};
use crate::models::*;

pub struct SystemClient<'a> {
    client: &'a Client,
}

impl<'a> SystemClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Check if the API is running correctly
    pub async fn health(&self) -> Result<HealthResponse> {
        self.client.get("/health").await
    }

    /// Get API version information
    pub async fn version(&self) -> Result<VersionResponse> {
        self.client.get("/version").await
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_system_paths() {
        let paths = vec!["/health", "/version"];

        for path in paths {
            assert!(path.starts_with("/"));
        }
    }

    #[test]
    fn test_version_path() {
        let path = "/version";
        assert_eq!(path, "/version");
    }
}
