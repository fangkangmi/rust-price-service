#[derive(Debug, PartialEq)]
pub enum Method {
    GET,
    POST,
    Uninitialized,
}

impl From<&str> for Method {
    fn from(s: &str) -> Self {
        match s {
            "GET" => Method::GET,
            "POST" => Method::POST,
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
pub enum Resourse {
    Path(String),
    Uninitialized,
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
        assert_eq!(Method::from("DELETE"), Method::Uninitialized);
    }

    #[test]
    fn test_version_into(){
        let v: Version = "HTTP/1.1".into();
        assert_eq!(v, Version::V1_1);
        assert_eq!(Version::from("HTTP/1.1"), Version::V1_1);
        assert_eq!(Version::from("HTTP/2.0"), Version::V2_0);
        assert_eq!(Version::from("HTTP/3.0"), Version::Uninitialized);
    }
}
