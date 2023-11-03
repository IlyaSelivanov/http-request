use std::fmt::Display;

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    RequestBuilder,
};

use crate::http_client::response::HttpResponse;

/// An enum representing the HTTP methods that can be used in an HTTP request.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl HttpMethod {
    /// Creates an `HttpMethod` instance from a string slice.
    pub fn from_str(method: &str) -> Option<Self> {
        match method {
            "GET" => Some(HttpMethod::Get),
            "POST" => Some(HttpMethod::Post),
            "PUT" => Some(HttpMethod::Put),
            "DELETE" => Some(HttpMethod::Delete),
            _ => None,
        }
    }
}

impl Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
        }
    }
}

/// Represents an HTTP request.
pub struct HttpRequest {
    method: HttpMethod,
    url: String,
    headers: Vec<(String, String)>,
    body: Option<String>,
}

/// Implementation of the `HttpRequest` struct.
impl HttpRequest {
    /// Creates a new `HttpClient` instance with the specified HTTP method and URL.
    pub fn new(method: HttpMethod, url: &str) -> Self {
        Self {
            method,
            url: url.to_string(),
            headers: Vec::new(),
            body: None,
        }
    }

    /// Adds a header to the HTTP request.
    ///
    /// # Arguments
    ///
    /// * `key` - A string slice that holds the header key.
    /// * `value` - A string slice that holds the header value.
    #[allow(dead_code)]
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.push((key.to_string(), value.to_string()));
    }

    /// Sets the body of the HTTP request.
    pub fn set_body(&mut self, body: &str) {
        self.body = Some(body.to_string());
    }

    /// Sends the HTTP request and returns the HTTP response.
    pub async fn send(&self) -> HttpResponse {
        let client = reqwest::Client::new();
        let request = reqwest::Request::new(
            self.method
                .to_string()
                .parse()
                .expect("Invalid HTTP method"),
            self.url.parse().expect("Invalid URL"),
        );
        let request_builder = RequestBuilder::from_parts(client, request);

        let mut headers = HeaderMap::new();
        for (key, value) in self.headers.clone() {
            let k = HeaderName::from_bytes(key.as_bytes()).unwrap();
            headers.insert(k, HeaderValue::from_str(value.as_str()).unwrap());
        }

        let request = request_builder
            .headers(headers)
            .body(self.body.clone().unwrap_or_default())
            .build()
            .unwrap();

        let response = reqwest::Client::new().execute(request).await.unwrap();

        HttpResponse::from_response(response).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_send() {
        let mut request = HttpRequest::new(HttpMethod::Get, "https://httpbin.org/get");
        request.add_header("User-Agent", "http-request");
        let response = request.send().await;
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn test_add_header() {
        let mut request = HttpRequest::new(HttpMethod::Get, "https://httpbin.org/get");
        request.add_header("User-Agent", "http-request");
        assert_eq!(request.headers.len(), 1);
    }

    #[test]
    fn test_set_body() {
        let mut request = HttpRequest::new(HttpMethod::Post, "https://httpbin.org/post");
        request.set_body("Hello, world!");
        assert_eq!(request.body, Some("Hello, world!".to_string()));
    }

    #[test]
    fn test_from_str() {
        assert_eq!(HttpMethod::from_str("GET"), Some(HttpMethod::Get));
        assert_eq!(HttpMethod::from_str("POST"), Some(HttpMethod::Post));
        assert_eq!(HttpMethod::from_str("PUT"), Some(HttpMethod::Put));
        assert_eq!(HttpMethod::from_str("DELETE"), Some(HttpMethod::Delete));
        assert_eq!(HttpMethod::from_str("INVALID"), None);
    }

    #[test]
    fn test_to_string() {
        assert_eq!(HttpMethod::Get.to_string(), "GET".to_string());
        assert_eq!(HttpMethod::Post.to_string(), "POST".to_string());
        assert_eq!(HttpMethod::Put.to_string(), "PUT".to_string());
        assert_eq!(HttpMethod::Delete.to_string(), "DELETE".to_string());
    }
}
