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

    pub fn from_response(response: reqwest::Response) -> Self {
        let status_code = response.status().as_u16();
        let headers = response.headers().clone();
        let body = response.bytes().unwrap().to_vec();

        Self {
            status_code,
            headers,
            body,
        }
    }
}
