use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub api_key: String,
    #[serde(default = "default_base_url")]
    pub base_url: String,
}

fn default_base_url() -> String {
    "https://api.forwardemail.net".to_string()
}

impl Config {
    pub fn load(api_key_override: Option<&str>) -> Result<Self> {
        let base_url =
            std::env::var("FORWARDEMAIL_BASE_URL").unwrap_or_else(|_| default_base_url());

        if let Some(key) = api_key_override {
            return Ok(Config {
                api_key: key.to_string(),
                base_url,
            });
        }

        if let Ok(key) = std::env::var("FORWARDEMAIL_API_KEY") {
            return Ok(Config {
                api_key: key,
                base_url,
            });
        }

        let path = Self::config_path()?;
        let contents = std::fs::read_to_string(&path).with_context(|| {
            format!(
                "No API key found. Create a config file at {} with:\n\n  api_key = \"your-api-key\"\n\nOr set FORWARDEMAIL_API_KEY environment variable.",
                path.display()
            )
        })?;

        let mut config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file at {}", path.display()))?;

        if std::env::var("FORWARDEMAIL_BASE_URL").is_ok() {
            config.base_url = base_url;
        }

        Ok(config)
    }

    pub fn base_url_only() -> String {
        std::env::var("FORWARDEMAIL_BASE_URL").unwrap_or_else(|_| default_base_url())
    }

    fn config_path() -> Result<PathBuf> {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        Ok(PathBuf::from(home).join(".config/forwardemail/config.toml"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serial_test::serial;

    #[test]
    fn test_cli_flag_override_takes_priority() {
        let config = Config::load(Some("flag-key")).unwrap();
        assert_eq!(config.api_key, "flag-key");
    }

    #[test]
    #[serial]
    fn test_env_var_override() {
        std::env::set_var("FORWARDEMAIL_API_KEY", "env-key");
        let config = Config::load(None).unwrap();
        assert_eq!(config.api_key, "env-key");
        std::env::remove_var("FORWARDEMAIL_API_KEY");
    }

    #[test]
    #[serial]
    fn test_base_url_env_override() {
        std::env::set_var("FORWARDEMAIL_BASE_URL", "http://localhost:9999");
        let config = Config::load(Some("key")).unwrap();
        assert_eq!(config.base_url, "http://localhost:9999");
        std::env::remove_var("FORWARDEMAIL_BASE_URL");
    }

    #[test]
    #[serial]
    fn test_default_base_url() {
        std::env::remove_var("FORWARDEMAIL_BASE_URL");
        let config = Config::load(Some("key")).unwrap();
        assert_eq!(config.base_url, "https://api.forwardemail.net");
    }

    #[test]
    #[serial]
    fn test_missing_api_key_errors() {
        std::env::remove_var("FORWARDEMAIL_API_KEY");
        let result = Config::load(None);
        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("No API key found") || err.contains("config"));
    }

    #[test]
    fn test_base_url_only() {
        let url = Config::base_url_only();
        assert!(!url.is_empty());
    }
}
