use reqwest::blocking::Client;
use std::collections::HashMap;
use std::time::Duration;

#[derive(Debug, Clone)]
pub struct FetchResult {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
    pub error: Option<String>,
}

#[derive(Clone)]
pub struct HttpClient {
    client: Client,
    timeout: Duration,
    user_agent: String,
}

impl HttpClient {
    pub fn new(timeout_seconds: u64, user_agent: &str) -> Result<Self, reqwest::Error> {
        let client = Client::builder().build()?;
        Ok(Self {
            client,
            timeout: Duration::from_secs(timeout_seconds),
            user_agent: user_agent.to_string(),
        })
    }

    pub fn fetch_bytes(
        &self,
        url: &str,
        accept: Option<&str>,
        if_none_match: Option<&str>,
        if_modified_since: Option<&str>,
        extra_headers: Option<&HashMap<String, String>>,
    ) -> FetchResult {
        let mut request = self
            .client
            .get(url)
            .header("User-Agent", &self.user_agent)
            .header(
                "Accept",
                accept.unwrap_or("text/markdown, text/plain;q=0.9, */*;q=0.8"),
            )
            .timeout(self.timeout);

        if let Some(value) = if_none_match {
            request = request.header("If-None-Match", value);
        }
        if let Some(value) = if_modified_since {
            request = request.header("If-Modified-Since", value);
        }
        if let Some(headers) = extra_headers {
            for (key, value) in headers {
                request = request.header(key, value);
            }
        }

        let response = match request.send() {
            Ok(response) => response,
            Err(err) => {
                return FetchResult {
                    status: 0,
                    headers: HashMap::new(),
                    body: None,
                    error: Some(err.to_string()),
                };
            }
        };

        let status = response.status().as_u16();
        let mut headers = HashMap::new();
        for (name, value) in response.headers().iter() {
            headers.insert(
                name.as_str().to_string(),
                value.to_str().unwrap_or("").to_string(),
            );
        }

        if status == 304 {
            return FetchResult {
                status,
                headers,
                body: None,
                error: None,
            };
        }

        if status != 200 {
            return FetchResult {
                status,
                headers,
                body: None,
                error: Some(format!("status={status}")),
            };
        }

        match response.bytes() {
            Ok(bytes) => FetchResult {
                status,
                headers,
                body: Some(bytes.to_vec()),
                error: None,
            },
            Err(err) => FetchResult {
                status,
                headers,
                body: None,
                error: Some(err.to_string()),
            },
        }
    }

    pub fn fetch_text(&self, url: &str) -> Result<String, String> {
        let result = self.fetch_bytes(url, None, None, None, None);
        if result.status != 200 || result.body.is_none() {
            return Err(format!(
                "Failed to fetch {}: status={} error={}",
                url,
                result.status,
                result.error.unwrap_or_default()
            ));
        }
        Ok(String::from_utf8_lossy(result.body.as_ref().unwrap()).to_string())
    }
}
