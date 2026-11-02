use figment::{Figment, providers::{Env, Format, Toml}};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::time::Duration;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    /// Base URL of the target site (e.g., https://otakudesu.blog)
    #[serde(default = "default_target_base_url")]
    pub target_base_url: String,

    /// Cloudflare Worker forward-proxy prefix (e.g., https://your-worker.workers.dev/?url=)
    #[serde(default)]
    pub cf_proxy_url: String,

    /// Request timeout in milliseconds
    #[serde(default = "default_request_timeout_ms")]
    pub request_timeout_ms: u64,

    /// Server bind address
    #[serde(default = "default_bind_addr")]
    pub bind_addr: String,

    /// Server port (HF Spaces sets PORT=7860)
    #[serde(default = "default_port")]
    pub port: u16,

    /// RUST_LOG level
    #[serde(default = "default_log_level")]
    pub log_level: String,
}

fn default_target_base_url() -> String {
    "https://otakudesu.blog".to_string()
}

fn default_request_timeout_ms() -> u64 {
    20000
}

fn default_bind_addr() -> String {
    "0.0.0.0".to_string()
}

fn default_port() -> u16 {
    7860
}

fn default_log_level() -> String {
    "info".to_string()
}

impl Config {
    pub fn load() -> anyhow::Result<Self> {
        let config: Config = Figment::new()
            .merge(Toml::file("Config.toml").nested())
            .merge(Env::prefixed("OTAKUDESU_").global())
            .extract()?;
        Ok(config)
    }

    pub fn request_timeout(&self) -> Duration {
        Duration::from_millis(self.request_timeout_ms)
    }

    pub fn target_url(&self, path: &str) -> String {
        let base = self.target_base_url.trim_end_matches('/');
        let path = path.trim_start_matches('/');
        if self.cf_proxy_url.is_empty() {
            format!("{}/{}", base, path)
        } else {
            let full = format!("{}/{}", base, path);
            format!("{}{}", self.cf_proxy_url.trim_end_matches('/'), urlencoding::encode(&full))
        }
    }

    pub fn ajax_url(&self) -> String {
        self.target_url("wp-admin/admin-ajax.php")
    }

    pub fn home_url(&self) -> String {
        self.target_url("/")
    }

    pub fn socket_addr(&self) -> std::net::SocketAddr {
        format!("{}:{}", self.bind_addr, self.port).parse().expect("invalid bind address")
    }
}

pub static CONFIG: Lazy<Config> = Lazy::new(|| {
    Config::load().expect("failed to load config")
});

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_url_without_proxy() {
        let config = Config {
            target_base_url: "https://otakudesu.blog".to_string(),
            cf_proxy_url: "".to_string(),
            request_timeout_ms: 20000,
            bind_addr: "0.0.0.0".to_string(),
            port: 7860,
            log_level: "info".to_string(),
        };
        assert_eq!(config.target_url("anime/test"), "https://otakudesu.blog/anime/test");
        assert_eq!(config.target_url("/anime/test/"), "https://otakudesu.blog/anime/test/");
    }

    #[test]
    fn test_target_url_with_proxy() {
        let config = Config {
            target_base_url: "https://otakudesu.blog".to_string(),
            cf_proxy_url: "https://proxy.workers.dev/?url=".to_string(),
            request_timeout_ms: 20000,
            bind_addr: "0.0.0.0".to_string(),
            port: 7860,
            log_level: "info".to_string(),
        };
        let url = config.target_url("anime/test");
        assert!(url.starts_with("https://proxy.workers.dev/?url="));
        assert!(url.contains("otakudesu.blog/anime/test"));
    }
}