#[derive(Default)]
pub struct Request {
    pub url: String,
    pub method: Option<Method>,
}

pub enum Method {
    Get,
    Post,
    Put,
    Delete,
}

impl Method {
    pub fn from_string(str: String) -> Option<Method> {
        match str.to_lowercase().as_str() {
            "get" => Some(Method::Get),
            "post" => Some(Method::Post),
            "put" => Some(Method::Put),
            "delete" => Some(Method::Delete),
            _ => panic!("No method associated with {}", str),
        }
    }
}
