use std::ops::Deref;

use crate::Cli;

#[derive(Default, Debug, Clone)]
pub struct Request {
    pub url: String,
    pub method: Option<Method>,
}

#[derive(Debug, Clone)]
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

impl Request {
    pub fn from_cli(cli: &Cli) -> Self {
        let url = match &cli.url {
            Some(u) => u.deref().to_string(),
            None => String::default(),
        };

        let method = match &cli.method {
            Some(m) => Method::from_string(m.deref().to_string()),
            None => None,
        };

        Request { url, method }
    }
}
