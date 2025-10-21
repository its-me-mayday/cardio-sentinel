use std::{env, net::{IpAddr, SocketAddr}, time::Duration};
use thiserror::Error;

#[derive(Clone, Debug)]
pub struct Config {
    pub app_name: String,
    pub http: HttpCfg,
    pub telemetry: TelemetryCfg,
}

#[derive(Clone, Debug)]
pub struct HttpCfg {
    pub host: IpAddr,
    pub port: u16,
    pub request_timeout: Duration,
    pub idle_timeout: Duration,
    pub body_limit_bytes: usize,
    pub allowed_origins: Vec<String>,
}

#[derive(Clone, Debug)]
pub struct TelemetryCfg {
    pub log_level: String,
}

#[derive(Debug, Error)]
pub enum ConfigError {
    #[error("invalid IP address in HTTP_HOST: {0}")]
    InvalidIp(String),
    #[error("invalid port in HTTP_PORT: {0}")]
    InvalidPort(String),
    #[error("invalid duration for {0}: {1}")]
    InvalidDuration(&'static str, String),
    #[error("invalid body limit: {0}")]
    InvalidBodyLimit(String),
    #[error("validation error: {0}")]
    Validation(String),
}

impl Config {
    pub fn from_env() -> Result<Self, ConfigError> {
        let _ = dotenvy::dotenv();

        let app_name = env_get_or("APP_NAME", "cardio-sentinel".into());

        let host_raw = env_get_or("HTTP_HOST", "0.0.0.0".into());
        let host: IpAddr = host_raw
            .parse()
            .map_err(|_| ConfigError::InvalidIp(host_raw.clone()))?;

        let port_raw = env_get_or("HTTP_PORT", "8080".into());
        let port: u16 = port_raw
            .parse()
            .map_err(|_| ConfigError::InvalidPort(port_raw.clone()))?;

        let req_to_raw = env_get_or("HTTP_REQUEST_TIMEOUT", "10s".into());
        let request_timeout = humantime::parse_duration(&req_to_raw)
            .map_err(|_| ConfigError::InvalidDuration("HTTP_REQUEST_TIMEOUT", req_to_raw.clone()))?;

        let idle_to_raw = env_get_or("HTTP_IDLE_TIMEOUT", "20s".into());
        let idle_timeout = humantime::parse_duration(&idle_to_raw)
            .map_err(|_| ConfigError::InvalidDuration("HTTP_IDLE_TIMEOUT", idle_to_raw.clone()))?;

        let body_raw = env_get_or("HTTP_BODY_LIMIT", "2MB".into());
        let body_limit_bytes = parse_size_bytes(&body_raw)
            .ok_or_else(|| ConfigError::InvalidBodyLimit(body_raw.clone()))?;
        
        let cors_raw = env_get_or("HTTP_CORS_ORIGINS", "".into());
        let allowed_origins = cors_raw
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect::<Vec<_>>();

        let log_level = env_get_or("RUST_LOG", "info".into());

        let http = HttpCfg {
            host,
            port,
            request_timeout,
            idle_timeout,
            body_limit_bytes,
            allowed_origins,
        };
        let telemetry = TelemetryCfg { log_level };

        let cfg = Self { app_name, http, telemetry };
        cfg.validate()?;
        Ok(cfg)
    }

    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.http.port == 0 {
            return Err(ConfigError::Validation("HTTP_PORT must be > 0".into()));
        }
        if self.http.request_timeout < Duration::from_millis(100) || self.http.request_timeout > Duration::from_secs(120) {
            return Err(ConfigError::Validation("HTTP_REQUEST_TIMEOUT must be between 100ms and 120s".into()));
        }
        if self.http.idle_timeout < Duration::from_secs(1) || self.http.idle_timeout > Duration::from_secs(300) {
            return Err(ConfigError::Validation("HTTP_IDLE_TIMEOUT must be between 1s and 300s".into()));
        }
        if self.http.body_limit_bytes < 8 * 1024 || self.http.body_limit_bytes > 32 * 1024 * 1024 {
            return Err(ConfigError::Validation("HTTP_BODY_LIMIT must be between 8KB and 32MB".into()));
        }
        Ok(())
    }
}

impl HttpCfg {
    pub fn bind_addr(&self) -> SocketAddr {
        SocketAddr::new(self.host, self.port)
    }
}

fn env_get_or(key: &str, default: String) -> String {
    env::var(key).unwrap_or(default)
}

fn parse_size_bytes(s: &str) -> Option<usize> {
    let lower = s.trim().to_ascii_lowercase();

    if let Ok(n) = lower.parse::<usize>() {
        return Some(n);
    }

    let (num_str, mul) = if let Some(n) = lower.strip_suffix("kb") {
        (n.trim(), 1024usize)
    } else if let Some(n) = lower.strip_suffix("mb") {
        (n.trim(), 1024usize * 1024)
    } else if let Some(n) = lower.strip_suffix("b") {
        (n.trim(), 1usize)
    } else {
        return None;
    };
    let base = num_str.parse::<f64>().ok()?;
    let val = (base * mul as f64).round() as usize;
    Some(val)
}
