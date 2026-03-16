use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct CatchAllPassword {
    #[serde(default)]
    pub id: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct CatchAllPasswordRow {
    #[tabled(rename = "ID")]
    pub id: String,
    #[tabled(rename = "DESCRIPTION")]
    pub description: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&CatchAllPassword> for CatchAllPasswordRow {
    fn from(c: &CatchAllPassword) -> Self {
        CatchAllPasswordRow {
            id: c.id.clone().unwrap_or("-".to_string()),
            description: c.description.clone().unwrap_or("-".to_string()),
            created_at: c.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_catch_all_password() {
        let json =
            r#"{"id":"pw123","description":"My catch-all","created_at":"2024-01-01T00:00:00Z"}"#;
        let cap: CatchAllPassword = serde_json::from_str(json).unwrap();
        assert_eq!(cap.id, Some("pw123".to_string()));
        assert_eq!(cap.description, Some("My catch-all".to_string()));
        let row = CatchAllPasswordRow::from(&cap);
        assert_eq!(row.id, "pw123");
        assert_eq!(row.description, "My catch-all");
    }
}
