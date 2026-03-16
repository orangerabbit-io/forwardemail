use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Member {
    pub id: String,
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
pub struct MemberRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "EMAIL")]
    pub email: String,
    #[tabled(rename = "GROUP")]
    pub group: String,
}

impl From<&Member> for MemberRow {
    fn from(m: &Member) -> Self {
        MemberRow {
            id: m.id.clone(),
            email: m.email.clone().unwrap_or("-".to_string()),
            group: m.group.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_member() {
        let json = r#"{"id":"member123","email":"user@example.com","group":"user","created_at":"2024-01-01T00:00:00Z"}"#;
        let member: Member = serde_json::from_str(json).unwrap();
        assert_eq!(member.id, "member123");
        assert_eq!(member.email, Some("user@example.com".to_string()));
        assert_eq!(member.group, Some("user".to_string()));
        let row = MemberRow::from(&member);
        assert_eq!(row.id, "member123");
        assert_eq!(row.email, "user@example.com");
    }
}
