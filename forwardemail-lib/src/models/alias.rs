use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Alias {
    pub id: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub recipients: Option<Vec<String>>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub labels: Option<Vec<String>>,
    #[serde(default)]
    pub is_enabled: Option<bool>,
    #[serde(default)]
    pub has_recipient_verification: Option<bool>,
    #[serde(default)]
    pub has_imap: Option<bool>,
    #[serde(default)]
    pub has_pgp: Option<bool>,
    #[serde(default)]
    pub error_code_if_disabled: Option<u32>,
    #[serde(default)]
    pub vacation_responder_is_enabled: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct AliasRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "RECIPIENTS")]
    pub recipients: String,
    #[tabled(rename = "ENABLED")]
    pub enabled: String,
    #[tabled(rename = "IMAP")]
    pub imap: String,
}

impl From<&Alias> for AliasRow {
    fn from(a: &Alias) -> Self {
        AliasRow {
            id: a.id.clone(),
            name: a.name.clone().unwrap_or("-".to_string()),
            recipients: a
                .recipients
                .as_ref()
                .map(|r| r.join(", "))
                .unwrap_or("-".to_string()),
            enabled: a
                .is_enabled
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
            imap: a
                .has_imap
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct GeneratedPassword {
    #[serde(default)]
    pub password: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_alias() {
        let json = r#"{"id":"alias123","name":"info","recipients":["user@gmail.com","other@gmail.com"],"is_enabled":true,"has_imap":false,"has_pgp":false,"created_at":"2024-01-01T00:00:00Z"}"#;
        let alias: Alias = serde_json::from_str(json).unwrap();
        assert_eq!(alias.id, "alias123");
        assert_eq!(alias.name, Some("info".to_string()));
        assert_eq!(alias.recipients.unwrap().len(), 2);
    }

    #[test]
    fn test_alias_row_formatting() {
        let alias = Alias {
            id: "a1".to_string(),
            name: Some("info".to_string()),
            domain: None,
            recipients: Some(vec!["a@b.com".to_string(), "c@d.com".to_string()]),
            description: None,
            labels: None,
            is_enabled: Some(true),
            has_recipient_verification: None,
            has_imap: Some(true),
            has_pgp: None,
            error_code_if_disabled: None,
            vacation_responder_is_enabled: None,
            created_at: None,
            updated_at: None,
        };
        let row = AliasRow::from(&alias);
        assert_eq!(row.recipients, "a@b.com, c@d.com");
        assert_eq!(row.enabled, "yes");
        assert_eq!(row.imap, "yes");
    }

    #[test]
    fn test_deserialize_minimal_alias() {
        let json = r#"{"id": "a1"}"#;
        let alias: Alias = serde_json::from_str(json).unwrap();
        assert_eq!(alias.name, None);
        let row = AliasRow::from(&alias);
        assert_eq!(row.name, "-");
    }
}
