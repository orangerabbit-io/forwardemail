use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Email {
    pub id: String,
    #[serde(default)]
    pub status: Option<String>,
    #[serde(default)]
    pub from: Option<String>,
    #[serde(default)]
    pub to: Option<Vec<String>>,
    #[serde(default)]
    pub cc: Option<Vec<String>>,
    #[serde(default)]
    pub bcc: Option<Vec<String>>,
    #[serde(default)]
    pub subject: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct EmailRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "STATUS")]
    pub status: String,
    #[tabled(rename = "FROM")]
    pub from: String,
    #[tabled(rename = "TO")]
    pub to: String,
    #[tabled(rename = "SUBJECT")]
    pub subject: String,
}

impl From<&Email> for EmailRow {
    fn from(e: &Email) -> Self {
        EmailRow {
            id: e.id.clone(),
            status: e.status.clone().unwrap_or("-".to_string()),
            from: e.from.clone().unwrap_or("-".to_string()),
            to: e
                .to
                .as_ref()
                .map(|t| t.join(", "))
                .unwrap_or("-".to_string()),
            subject: e.subject.clone().unwrap_or("-".to_string()),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct EmailLimit {
    pub count: u32,
    pub limit: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_email() {
        let json = r#"{"id":"email123","status":"queued","from":"sender@example.com","to":["recipient@example.com"],"subject":"Hello","created_at":"2024-01-01T00:00:00Z"}"#;
        let email: Email = serde_json::from_str(json).unwrap();
        assert_eq!(email.id, "email123");
        assert_eq!(email.status, Some("queued".to_string()));
        assert_eq!(email.to.unwrap().len(), 1);
    }

    #[test]
    fn test_deserialize_email_limit() {
        let json = r#"{"count": 5, "limit": 300}"#;
        let limit: EmailLimit = serde_json::from_str(json).unwrap();
        assert_eq!(limit.count, 5);
        assert_eq!(limit.limit, 300);
    }

    #[test]
    fn test_email_row() {
        let email = Email {
            id: "e1".to_string(),
            status: Some("sent".to_string()),
            from: Some("a@b.com".to_string()),
            to: Some(vec!["c@d.com".to_string()]),
            cc: None,
            bcc: None,
            subject: Some("Test".to_string()),
            created_at: None,
            updated_at: None,
        };
        let row = EmailRow::from(&email);
        assert_eq!(row.status, "sent");
        assert_eq!(row.to, "c@d.com");
    }
}
