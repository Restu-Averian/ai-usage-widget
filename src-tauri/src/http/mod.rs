use std::collections::HashMap;
use std::time::Duration;

use async_trait::async_trait;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
}

#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: HttpMethod,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<Vec<u8>>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: Vec<(String, String)>,
    pub body: Vec<u8>,
    pub body_truncated: bool,
}

impl HttpResponse {
    pub fn bounded(
        status: u16,
        headers: Vec<(String, String)>,
        body: Vec<u8>,
        max_body_bytes: usize,
    ) -> Self {
        let body_truncated = body.len() > max_body_bytes;
        let body = body.into_iter().take(max_body_bytes).collect();

        Self {
            status,
            headers,
            body,
            body_truncated,
        }
    }
}

#[derive(Debug, Error, PartialEq, Eq)]
pub enum HttpError {
    #[error("only HTTPS provider requests are allowed")]
    InsecureUrl,
    #[error("HTTP request failed")]
    RequestFailed,
    #[error("invalid HTTP header")]
    InvalidHeader,
}

#[async_trait]
pub trait HttpTransport: Send + Sync {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, HttpError>;
}

#[derive(Clone)]
pub struct ReqwestHttpTransport {
    client: reqwest::Client,
    max_body_bytes: usize,
}

impl ReqwestHttpTransport {
    pub fn new(timeout: Duration, max_body_bytes: usize) -> Result<Self, HttpError> {
        let client = reqwest::Client::builder()
            .timeout(timeout)
            .build()
            .map_err(|_| HttpError::RequestFailed)?;

        Ok(Self {
            client,
            max_body_bytes,
        })
    }
}

#[async_trait]
impl HttpTransport for ReqwestHttpTransport {
    async fn send(&self, request: HttpRequest) -> Result<HttpResponse, HttpError> {
        if !request.url.starts_with("https://") {
            return Err(HttpError::InsecureUrl);
        }

        let method = match request.method {
            HttpMethod::Get => reqwest::Method::GET,
            HttpMethod::Post => reqwest::Method::POST,
        };
        let mut builder = self.client.request(method, request.url);

        for (key, value) in request.headers {
            builder = builder.header(key, value);
        }

        if let Some(body) = request.body {
            builder = builder.body(body);
        }

        let response = builder.send().await.map_err(|_| HttpError::RequestFailed)?;
        let status = response.status().as_u16();
        let headers = response
            .headers()
            .iter()
            .map(|(key, value)| {
                Ok((
                    key.as_str().to_string(),
                    value
                        .to_str()
                        .map_err(|_| HttpError::InvalidHeader)?
                        .to_string(),
                ))
            })
            .collect::<Result<Vec<_>, HttpError>>()?;
        let body = response
            .bytes()
            .await
            .map_err(|_| HttpError::RequestFailed)?
            .to_vec();

        Ok(HttpResponse::bounded(
            status,
            headers,
            body,
            self.max_body_bytes,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn rejects_non_https_requests() {
        let client = ReqwestHttpTransport::new(Duration::from_secs(1), 1024).expect("client");
        let request = HttpRequest {
            method: HttpMethod::Get,
            url: "http://example.test".to_string(),
            headers: Default::default(),
            body: None,
        };

        let result = client.send(request).await;

        assert!(matches!(result, Err(HttpError::InsecureUrl)));
    }

    #[test]
    fn caps_response_body() {
        let response = HttpResponse::bounded(200, vec![], b"123456".to_vec(), 4);

        assert_eq!(response.body, b"1234");
        assert!(response.body_truncated);
    }
}
