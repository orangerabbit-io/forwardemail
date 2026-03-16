use serde::{Deserialize, Serialize};
use tabled::Tabled;

#[derive(Debug, Deserialize, Serialize)]
pub struct Domain {
    pub name: String,
    #[serde(default)]
    pub plan: Option<String>,
    #[serde(default)]
    pub max_recipients_per_alias: Option<u32>,
    #[serde(default)]
    pub smtp_port: Option<String>,
    #[serde(default)]
    pub has_adult_content_protection: Option<bool>,
    #[serde(default)]
    pub has_phishing_protection: Option<bool>,
    #[serde(default)]
    pub has_executable_protection: Option<bool>,
    #[serde(default)]
    pub has_virus_protection: Option<bool>,
    #[serde(default)]
    pub has_recipient_verification: Option<bool>,
    #[serde(default)]
    pub retention_days: Option<u32>,
    #[serde(default)]
    pub has_mx_record: Option<bool>,
    #[serde(default)]
    pub has_txt_record: Option<bool>,
    #[serde(default)]
    pub created_at: Option<String>,
    #[serde(default)]
    pub updated_at: Option<String>,
}

#[derive(Debug, Tabled)]
pub struct DomainRow {
    #[tabled(rename = "DOMAIN")]
    pub name: String,
    #[tabled(rename = "PLAN")]
    pub plan: String,
    #[tabled(rename = "MX")]
    pub mx: String,
    #[tabled(rename = "TXT")]
    pub txt: String,
    #[tabled(rename = "CREATED")]
    pub created_at: String,
}

impl From<&Domain> for DomainRow {
    fn from(d: &Domain) -> Self {
        DomainRow {
            name: d.name.clone(),
            plan: d.plan.clone().unwrap_or("-".to_string()),
            mx: d
                .has_mx_record
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
            txt: d
                .has_txt_record
                .map(|b| if b { "yes" } else { "no" })
                .unwrap_or("-")
                .to_string(),
            created_at: d.created_at.clone().unwrap_or("-".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_domain() {
        let json = r#"{"name":"example.com","plan":"enhanced_protection","has_mx_record":true,"has_txt_record":true,"retention_days":30,"created_at":"2024-01-01T00:00:00Z"}"#;
        let domain: Domain = serde_json::from_str(json).unwrap();
        assert_eq!(domain.name, "example.com");
        assert_eq!(domain.has_mx_record, Some(true));
        assert_eq!(domain.retention_days, Some(30));
    }

    #[test]
    fn test_domain_row() {
        let domain = Domain {
            name: "example.com".to_string(),
            plan: Some("free".to_string()),
            max_recipients_per_alias: None,
            smtp_port: None,
            has_adult_content_protection: None,
            has_phishing_protection: None,
            has_executable_protection: None,
            has_virus_protection: None,
            has_recipient_verification: None,
            retention_days: None,
            has_mx_record: Some(true),
            has_txt_record: Some(false),
            created_at: Some("2024-01-01".to_string()),
            updated_at: None,
        };
        let row = DomainRow::from(&domain);
        assert_eq!(row.mx, "yes");
        assert_eq!(row.txt, "no");
    }
}
