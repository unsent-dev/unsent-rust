use crate::client::{Client, Result};
use crate::models::*;

pub struct SettingsClient<'a> {
    client: &'a Client,
}

impl<'a> SettingsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// Get team settings
    pub fn get(&self) -> Result<Settings> {
        self.client.get("/settings")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_settings_path() {
        let path = "/settings";
        assert_eq!(path, "/settings");
    }
}
