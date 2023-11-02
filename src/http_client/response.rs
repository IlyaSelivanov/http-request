use std::collections::HashMap;

pub struct HttpResponse {
    pub status_code: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

impl HttpResponse {
    pub fn new() -> Self {
        Self {
            status_code: 0,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub async fn from_response(response: reqwest::Response) -> Self {
        let mut http_response = Self::new();

        http_response.status_code = response.status().as_u16();

        for (key, value) in response.headers().iter() {
            http_response
                .headers
                .insert(key.to_string(), value.to_str().unwrap().to_string());
        }

        http_response.body = response.bytes().await.unwrap().to_vec();

        http_response
    }
}
