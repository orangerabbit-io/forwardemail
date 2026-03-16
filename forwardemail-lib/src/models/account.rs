use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Account {
    #[serde(default)]
    pub email: Option<String>,
    #[serde(default)]
    pub given_name: Option<String>,
    #[serde(default)]
    pub family_name: Option<String>,
    #[serde(default)]
    pub avatar_url: Option<String>,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct AccountRow {
    #[tabled(rename = "EMAIL")]
    pub email: String,
    #[tabled(rename = "NAME")]
    pub name: String,
    #[tabled(rename = "PLAN")]
    pub plan: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&Account> for AccountRow {
    fn from(a: &Account) -> Self {
        let name = match (&a.given_name, &a.family_name) {
            (Some(g), Some(f)) => format!("{} {}", g, f),
            (Some(g), None) => g.clone(),
            (None, Some(f)) => f.clone(),
            (None, None) => "-".to_string(),
        };
        AccountRow {
            email: a.email.clone().unwrap_or("-".to_string()),
            name,
            plan: a.plan.clone().unwrap_or("-".to_string()),
            created_at: a.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_account() {
        let json = r#"{"email":"user@example.com","given_name":"John","family_name":"Doe","plan":"enhanced_protection","created_at":"2024-01-01T00:00:00Z","updated_at":"2024-06-01T00:00:00Z"}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.email, Some("user@example.com".to_string()));
        assert_eq!(account.plan, Some("enhanced_protection".to_string()));
    }

    #[test]
    fn test_account_row_full_name() {
        let account = Account {
            email: Some("user@example.com".to_string()),
            given_name: Some("John".to_string()),
            family_name: Some("Doe".to_string()),
            avatar_url: None,
            plan: Some("free".to_string()),
            created_at: Some("2024-01-01".to_string()),
            updated_at: None,
        };
        let row = AccountRow::from(&account);
        assert_eq!(row.name, "John Doe");
    }

    #[test]
    fn test_deserialize_minimal_account() {
        let json = r#"{}"#;
        let account: Account = serde_json::from_str(json).unwrap();
        assert_eq!(account.email, None);
        let row = AccountRow::from(&account);
        assert_eq!(row.email, "-");
        assert_eq!(row.name, "-");
    }
}
