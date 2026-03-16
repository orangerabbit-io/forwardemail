use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct EncryptRequest {
    pub input: String,
}

#[derive(Debug, Deserialize)]
pub struct EncryptResponse {
    #[serde(default)]
    pub encrypted: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deserialize_encrypt_response() {
        let json = r#"{"encrypted":"enc:abc123"}"#;
        let resp: EncryptResponse = serde_json::from_str(json).unwrap();
        assert_eq!(resp.encrypted, Some("enc:abc123".to_string()));
    }
}
