use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Invite {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub group: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct InviteRow {
    #[tabled(rename = "EMAIL")]
    pub email: String,
    #[tabled(rename = "GROUP")]
    pub group: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&Invite> for InviteRow {
    fn from(i: &Invite) -> Self {
        InviteRow {
            email: i.email.clone().unwrap_or("-".to_string()),
            group: i.group.clone().unwrap_or("-".to_string()),
            created_at: i.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_invite() {
        let json = r#"{"email":"user@example.com","group":"admin","created_at":"2024-01-01T00:00:00Z"}"#;
        let invite: Invite = serde_json::from_str(json).unwrap();
        assert_eq!(invite.email, Some("user@example.com".to_string()));
        assert_eq!(invite.group, Some("admin".to_string()));
        let row = InviteRow::from(&invite);
        assert_eq!(row.email, "user@example.com");
        assert_eq!(row.group, "admin");
    }
}
