use crate::config::CONFIG;
use anyhow::{Context, Result};
use base64::{Engine as _, engine::general_purpose};
use reqwest::{
    Client, ClientBuilder, Response,
    cookie::{CookieStore, Jar},
    header::{HeaderMap, HeaderValue, ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, COOKIE, USER_AGENT},
};
use std::sync::Arc;
use std::time::Duration;
use url::Url;

/// Browser-like headers to mimic a real browser request
fn browser_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/126.0.0.0 Safari/537.36",
        ),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"),
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_static("id-ID,id;q=0.9,en-US;q=0.8,en;q=0.7"),
    );
    headers
}

/// Application state shared across handlers
#[derive(Clone)]
pub struct AppState {
    pub client: Client,
    pub cookie_jar: Arc<Jar>,
}

impl AppState {
    pub fn new() -> Result<Self> {
        let cookie_jar = Arc::new(Jar::default());
        let client = ClientBuilder::new()
            .default_headers(browser_headers())
            .cookie_provider(Arc::clone(&cookie_jar))
            .timeout(CONFIG.request_timeout())
            .gzip(true)
            .brotli(true)
            .build()
            .context("failed to build reqwest client")?;
        Ok(Self { client, cookie_jar })
    }

    /// Fetch raw HTML for a given path (relative to TARGET_BASE_URL) or absolute URL
    pub async fn fetch_html(&self, path: &str) -> Result<String> {
        let url = CONFIG.target_url(path);
        let res = self
            .client
            .get(&url)
            .send()
            .await
            .with_context(|| format!("failed to fetch {}", url))?;

        if !res.status().is_success() {
            anyhow::bail!("upstream returned {} for {}", res.status(), url);
        }

        let text = res.text().await.with_context(|| format!("failed to read body from {}", url))?;
        Ok(text)
    }

    /// POST to admin-ajax.php with form data, using shared cookie jar
    pub async fn post_admin_ajax(&self, form_data: &[(&str, &str)]) -> Result<serde_json::Value> {
        let url = CONFIG.ajax_url();
        let body = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", k, urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");

        let res = self
            .client
            .post(&url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(body)
            .send()
            .await
            .with_context(|| format!("failed to POST to {}", url))?;

        if !res.status().is_success() {
            anyhow::bail!("admin-ajax.php returned {}", res.status());
        }

        let json = res
            .json::<serde_json::Value>()
            .await
            .with_context(|| "failed to parse admin-ajax JSON response")?;
        Ok(json)
    }

    /// Resolve a mirror's data_content (base64 encoded JSON) into embeddable iframe HTML
    /// Replicates the site's two-step WordPress admin-ajax flow:
    /// 1. GET homepage to obtain session cookies
    /// 2. POST action=aa1208... (no params) -> returns a nonce
    /// 3. POST decoded {id,i,q} + nonce + action=2a3505... -> returns base64 HTML
    pub async fn resolve_mirror_embed(&self, data_content: &str) -> Result<String> {
        // Decode base64 JSON: { "id": "...", "i": "...", "q": "..." }
        let decoded_json = general_purpose::STANDARD
            .decode(data_content)
            .context("failed to base64 decode data_content")?;
        let decoded_str = String::from_utf8(decoded_json).context("data_content not valid UTF-8")?;
        let decoded: serde_json::Value = serde_json::from_str(&decoded_str).context("data_content not valid JSON")?;

        // Extract required fields
        let id = decoded["id"].as_str().context("missing 'id' in data_content")?;
        let i = decoded["i"].as_str().context("missing 'i' in data_content")?;
        let q = decoded["q"].as_str().context("missing 'q' in data_content")?;

        // Step 1: GET homepage to establish session cookies
        let home_url = CONFIG.home_url();
        let _ = self.client.get(&home_url).send().await?.text().await?;

        // Step 2: POST to get nonce (action=aa1208d27f29ca340c92c66d1926f13f)
        let nonce_resp = self
            .post_admin_ajax(&[("action", "aa1208d27f29ca340c92c66d1926f13f")])
            .await?;
        let nonce = nonce_resp["data"]
            .as_str()
            .context("failed to obtain nonce from admin-ajax")?
            .to_string();

        // Step 3: POST with decoded params + nonce to get embed HTML (action=2a3505c93b0035d3f455df82bf976b84)
        let embed_resp = self
            .post_admin_ajax(&[
                ("action", "2a3505c93b0035d3f455df82bf976b84"),
                ("id", id),
                ("i", i),
                ("q", q),
                ("nonce", &nonce),
            ])
            .await?;

        let html_b64 = embed_resp["data"]
            .as_str()
            .context("missing embed HTML in response")?;

        let embed_html = general_purpose::STANDARD
            .decode(html_b64)
            .context("failed to base64 decode embed HTML")?;
        String::from_utf8(embed_html).context("embed HTML not valid UTF-8")
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new().expect("failed to create AppState")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::{matchers::*, Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_resolve_mirror_embed_mock() {
        let mock_server = MockServer::start().await;

        // Mock homepage
        Mock::given(method("GET"))
            .and(path("/"))
            .respond_with(ResponseTemplate::new(200).set_body_string(""))
            .mount(&mock_server)
            .await;

        // Mock nonce request
        Mock::given(method("POST"))
            .and(path("/wp-admin/admin-ajax.php"))
            .and(body_string_contains("action=aa1208d27f29ca340c92c66d1926f13f"))
            .respond_with(
                ResponseTemplate::new(200).set_body_json(serde_json::json!({"data": "test-nonce-123"})),
            )
            .mount(&mock_server)
            .await;

        // Mock embed request
        Mock::given(method("POST"))
            .and(path("/wp-admin/admin-ajax.php"))
            .and(body_string_contains("action=2a3505c93b0035d3f455df82bf976b84"))
            .and(body_string_contains("nonce=test-nonce-123"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
                "data": general_purpose::STANDARD.encode("<iframe src='https://example.com/embed'></iframe>")
            })))
            .mount(&mock_server)
            .await;

        // Override config for test
        let client = ClientBuilder::new()
            .default_headers(browser_headers())
            .cookie_provider(Arc::new(Jar::default()))
            .build()
            .unwrap();

        // This test would need config override - skipping full integration test
        // Real integration test would use CONFIG override
    }
}