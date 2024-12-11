use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    PATCH,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
            "PUT" => Method::PUT,
            "DELETE" => Method::DELETE,
            "PATCH" => Method::PATCH,
            _ => Method::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Version {
    V1_1,
    V2_0,
    Uninitialized,
}

impl From<&str> for Version {
    fn from(s: &str) -> Self {
        match s {
            "HTTP/1.1" => Version::V1_1,
            "HTTP/2.0" => Version::V2_0,
            _ => Version::Uninitialized,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Resource {
    Path(String),
    Uninitialized,
}

#[derive(Debug, PartialEq)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    pub headers: HashMap<String, String>,
    pub message_body: Vec<u8>,
}

impl FromStr for HttpRequest {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let mut headers = HashMap::new();
        let mut message_body = Vec::new();
        let mut method = Method::Uninitialized;
        let mut version = Version::Uninitialized;
        let mut resource = Resource::Uninitialized;

        if let Some(request_line) = lines.next() {
            let mut parts = request_line.split_whitespace();
            method = Method::from(parts.next().ok_or("Missing method")?);
            resource = Resource::Path(parts.next().ok_or("Missing resource")?.to_string());
            version = Version::from(parts.next().ok_or("Missing version")?);
        }

        for line in lines.clone() {
            if line.is_empty() {
                break;
            }
            let mut parts = line.split(": ");
            headers.insert(parts.next().ok_or("Malformed header")?.to_string(), parts.next().ok_or("Malformed header value")?.to_string());
        }

        let mut message_body_started = false;
        for line in lines {
            if message_body_started {
                message_body.extend_from_slice(line.as_bytes());
            } else if line.is_empty() {
                message_body_started = true;
            }
        }

        Ok(HttpRequest {
            method,
            version,
            resource,
            headers,
            message_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_method_from() {
        let m: Method = "GET".into();
        assert_eq!(m, Method::GET);
        assert_eq!(Method::from("GET"), Method::GET);
        assert_eq!(Method::from("POST"), Method::POST);
        assert_eq!(Method::from("PUT"), Method::PUT);
        assert_eq!(Method::from("DELETE"), Method::DELETE);
        assert_eq!(Method::from("PATCH"), Method::PATCH);
        assert_eq!(Method::from("UNKNOWN"), Method::Uninitialized);
    }

    #[test]
    fn test_version_into(){
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("HTTP/3.0"), Version::Uninitialized);
    }

    #[test]
    fn test_http_request_from_str() {
        let request_str = "GET /path HTTP/1.1\r\nHost: example.com\r\n\r\nbody";
        let request: HttpRequest = request_str.parse().unwrap();
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.resource, Resource::Path("/path".to_string()));
        assert_eq!(request.headers.get("Host"), Some(&"example.com".to_string()));
        assert_eq!(request.message_body, b"body".to_vec());
    }

    #[test]
    fn test_read_http_request() {
        let request_str = "GET /path HTTP/1.1\r\nHost: example.com\r\nUser-Agent: curl/7.71.1\r\n\r\nbody";
        let request: HttpRequest = request_str.parse().unwrap();
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.version, Version::V1_1);
        assert_eq!(request.resource, Resource::Path("/path".to_string()));
        assert_eq!(request.headers.get("Host"), Some(&"example.com".to_string()));
        assert_eq!(request.message_body, b"body".to_vec());
    }
}