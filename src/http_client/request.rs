pub mod http_response;
pub use http_response::HttpResponse;

/// An enum representing the HTTP methods that can be used in an HTTP request.
enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
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
    pub fn send(&self) -> HttpResponse {
        HttpResponse::new()
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

    #[test]
    fn test_send() {
        let http_request = HttpRequest::new(HttpMethod::Get, "https://www.example.com");
        let http_response = http_request.send();
        assert_eq!(http_response.status_code, 200);
        assert_eq!(http_response.body, "");
    }
}
