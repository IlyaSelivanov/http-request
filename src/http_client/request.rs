use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    RequestBuilder,
};

use crate::{http_client::response::HttpResponse, request};

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

    /// Returns the string representation of the HTTP method.
    pub fn to_string(&self) -> String {
        match self {
            HttpMethod::Get => "GET".to_string(),
            HttpMethod::Post => "POST".to_string(),
            HttpMethod::Put => "PUT".to_string(),
            HttpMethod::Delete => "DELETE".to_string(),
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

    #[test]
    fn test_new_http_request() {
        let http_request = HttpRequest::new(HttpMethod::Get, "https://www.example.com");
        assert_eq!(http_request.method, HttpMethod::Get);
        assert_eq!(http_request.url, "https://www.example.com");
        assert_eq!(http_request.headers.len(), 0);
        assert_eq!(http_request.body, None);
    }

    #[test]
    fn test_add_header() {
        let mut http_request = HttpRequest::new(HttpMethod::Get, "https://www.example.com");
        http_request.add_header("Content-Type", "application/json");
        assert_eq!(http_request.headers.len(), 1);
        assert_eq!(
            http_request.headers[0],
            ("Content-Type".to_string(), "application/json".to_string())
        );
    }

    #[test]
    fn test_set_body() {
        let mut http_request = HttpRequest::new(HttpMethod::Post, "https://www.example.com");
        http_request.set_body(r#"{"name": "John Doe", "age": 30}"#);
        assert_eq!(
            http_request.body,
            Some(r#"{"name": "John Doe", "age": 30}"#.to_string())
        );
    }

    // #[test]
    // fn test_send() {
    //     let http_request = HttpRequest::new(HttpMethod::Get, "https://www.example.com");
    //     let http_response = http_request.send().await;
    //     assert_eq!(http_response.status_code, 200);
    //     assert_eq!(http_response.body, "");
    // }
}
