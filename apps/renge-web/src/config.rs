use std::{env, error::Error, fmt, str::FromStr};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum LogFormat {
    Pretty,
    Json,
}

impl FromStr for LogFormat {
    type Err = ConfigError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "pretty" => Ok(Self::Pretty),
            "json" => Ok(Self::Json),
            _ => Err(ConfigError::InvalidLogFormat(value.to_owned())),
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct AppConfig {
    pub(crate) log_format: LogFormat,
    pub(crate) trust_proxy_headers: bool,
}

impl AppConfig {
    pub(crate) fn from_env() -> Result<Self, ConfigError> {
        let log_format = env::var("LOG_FORMAT").ok();
        let trust_proxy_headers = env::var("TRUST_PROXY_HEADERS").ok();
        Self::from_values(log_format.as_deref(), trust_proxy_headers.as_deref())
    }

    fn from_values(
        log_format: Option<&str>,
        trust_proxy_headers: Option<&str>,
    ) -> Result<Self, ConfigError> {
        let log_format = log_format.unwrap_or("pretty").parse()?;
        let trust_proxy_headers = trust_proxy_headers
            .map(|value| {
                value
                    .parse()
                    .map_err(|_| ConfigError::InvalidTrustProxyHeaders(value.to_owned()))
            })
            .transpose()?
            .unwrap_or(false);

        Ok(Self {
            log_format,
            trust_proxy_headers,
        })
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) enum ConfigError {
    InvalidLogFormat(String),
    InvalidTrustProxyHeaders(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLogFormat(value) => {
                write!(
                    f,
                    "LOG_FORMAT は pretty または json を指定してください: {value}"
                )
            }
            Self::InvalidTrustProxyHeaders(value) => {
                write!(
                    f,
                    "TRUST_PROXY_HEADERS は true または false を指定してください: {value}"
                )
            }
        }
    }
}

impl Error for ConfigError {}

#[cfg(test)]
mod tests {
    use super::{AppConfig, ConfigError, LogFormat};

    #[test]
    fn defaults_to_pretty_logs_and_untrusted_proxy_headers() {
        let config = AppConfig::from_values(None, None).unwrap();

        assert_eq!(config.log_format, LogFormat::Pretty);
        assert!(!config.trust_proxy_headers);
    }

    #[test]
    fn accepts_json_logs_and_trusted_proxy_headers() {
        let config = AppConfig::from_values(Some("json"), Some("true")).unwrap();

        assert_eq!(config.log_format, LogFormat::Json);
        assert!(config.trust_proxy_headers);
    }

    #[test]
    fn rejects_invalid_log_format() {
        let error = AppConfig::from_values(Some("text"), None).unwrap_err();

        assert_eq!(error, ConfigError::InvalidLogFormat("text".to_owned()));
    }

    #[test]
    fn rejects_invalid_proxy_header_setting() {
        let error = AppConfig::from_values(None, Some("yes")).unwrap_err();

        assert_eq!(
            error,
            ConfigError::InvalidTrustProxyHeaders("yes".to_owned())
        );
    }
}
