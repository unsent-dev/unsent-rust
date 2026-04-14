// @manual
use crate::client::{Client, Result};
use crate::models::*;

pub struct TeamsClient<'a> {
    client: &'a Client,
}

impl<'a> TeamsClient<'a> {
    pub fn new(client: &'a Client) -> Self {
        Self { client }
    }

    /// List all teams
    pub async fn list(&self) -> Result<Vec<Team>> {
        self.client.get("/teams").await
    }

    /// Get current team information
    pub async fn get(&self) -> Result<Team> {
        self.client.get("/team").await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_teams_client_creation() {
        let path = "/teams";
        assert_eq!(path, "/teams");
    }

    #[test]
    fn test_teams_list_path() {
        let path = "/teams";
        assert_eq!(path, "/teams");
    }

    #[test]
    fn test_team_get_path() {
        let path = "/team";
        assert_eq!(path, "/team");
    }

    #[test]
    fn test_team_struct() {
        let team = Team {
            id: "team-123".to_string(),
            name: "Test Team".to_string(),
            plan: "pro".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        assert_eq!(team.id, "team-123");
        assert_eq!(team.name, "Test Team");
        assert_eq!(team.plan, "pro");
    }

    #[test]
    fn test_teams_list_response_deserialization() {
        let json = r#"{"teams": []}"#;
        let response: std::result::Result<TeamsListResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());

        let teams = response.unwrap();
        assert_eq!(teams.teams.len(), 0);
    }

    #[test]
    fn test_teams_list_response_with_data() {
        let json = r#"{"teams": [{"id": "1", "name": "Team A", "plan": "free", "createdAt": "2024-01-01"}]}"#;
        let response: std::result::Result<TeamsListResponse, _> = serde_json::from_str(json);
        assert!(response.is_ok());

        let teams = response.unwrap();
        assert_eq!(teams.teams.len(), 1);
        assert_eq!(teams.teams[0].name, "Team A");
    }

    #[test]
    fn test_team_serialization() {
        let team = Team {
            id: "team-123".to_string(),
            name: "Test Team".to_string(),
            plan: "enterprise".to_string(),
            created_at: "2024-01-01T00:00:00Z".to_string(),
        };

        let json = serde_json::to_string(&team).unwrap();
        assert!(json.contains("team-123"));
        assert!(json.contains("Test Team"));
    }
}
