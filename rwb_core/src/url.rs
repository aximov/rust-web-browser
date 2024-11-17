use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct Url {
    url: String,
    scheme: String,
    host: String,
    port: Option<u16>,
    path: String,
    query: Option<String>,
}

impl Url {
    pub fn new(url: String) -> Result<Self, String> {
        let scheme = Self::parse_scheme(&url);
        if scheme != "http" {
            return Err(format!(
                "Unsupported scheme: {}. Only 'http' is allowed.",
                scheme
            ));
        }

        Ok(Self {
            url: url.clone(),
            scheme,
            host: Self::parse_host(&url),
            port: Self::parse_port(&url),
            path: Self::parse_path(&url),
            query: Self::parse_query(&url),
        })
    }

    fn parse_scheme(url: &str) -> String {
        let url_parts: Vec<&str> = url.split("://").collect();
        if url_parts.len() > 1 {
            return url_parts[0].to_string();
        }
        String::new()
    }

    fn parse_host(url: &str) -> String {
        let url_parts: Vec<&str> = url.split("://").collect();
        if url_parts.len() > 1 {
            if let Some(index) = url_parts[1].find(':') {
                return url_parts[1][..index].to_string();
            }
            if let Some(index) = url_parts[1].find('/') {
                return url_parts[1][..index].to_string();
            }
        }
        String::new()
    }

    fn parse_port(url: &str) -> Option<u16> {
        let url_parts: Vec<&str> = url.split("://").collect();
        if url_parts.len() > 1 {
            let host_and_path = url_parts[1];
            if let Some(colon_index) = host_and_path.find(':') {
                let rest = &host_and_path[colon_index + 1..];
                if let Some(end_index) = rest.find(|c| c == '/' || c == '?') {
                    if let Ok(port) = rest[..end_index].parse::<u16>() {
                        return Some(port);
                    }
                } else if let Ok(port) = rest.parse::<u16>() {
                    return Some(port);
                }
            }
        }
        Some(80)
    }

    fn parse_path(url: &str) -> String {
        let url_parts: Vec<&str> = url.split("://").collect();
        if url_parts.len() > 1 {
            if let Some(index) = url_parts[1].find('/') {
                let path_and_query = url_parts[1][index..].to_string();
                if let Some(index) = path_and_query.find('?') {
                    return path_and_query[..index].to_string();
                }
                return path_and_query;
            }
        }
        "/".to_string()
    }

    fn parse_query(url: &str) -> Option<String> {
        let url_parts: Vec<&str> = url.split("://").collect();
        if url_parts.len() > 1 {
            if let Some(index) = url_parts[1].find('?') {
                return Some(url_parts[1][index + 1..].to_string());
            }
        }
        None
    }

    pub fn scheme(&self) -> String {
        self.scheme.clone()
    }

    pub fn host(&self) -> String {
        self.host.clone()
    }

    pub fn port(&self) -> u16 {
        self.port.unwrap()
    }

    pub fn path(&self) -> String {
        self.path.clone()
    }

    pub fn query(&self) -> Option<String> {
        self.query.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_http_url() {
        let url = Url::new("http://example.com".to_string());
        assert!(url.is_ok());
        assert_eq!(url.unwrap().scheme, "http");
    }

    #[test]
    fn test_invalid_scheme() {
        let url = Url::new("https://example.com".to_string());
        assert!(url.is_err());
        assert_eq!(
            url.err().unwrap(),
            "Unsupported scheme: https. Only 'http' is allowed."
        );
    }

    #[test]
    fn test_missing_scheme() {
        let url = Url::new("example.com".to_string());
        assert!(url.is_err());
        assert_eq!(
            url.err().unwrap(),
            "Unsupported scheme: . Only 'http' is allowed."
        );
    }

    #[test]
    fn test_url_host_port() {
        let url = Url::new("http://example.com:8080".to_string());
        assert!(url.is_ok());
        let url = url.unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port.unwrap(), 8080);
    }

    #[test]
    fn test_url_host_port_path() {
        let url = Url::new("http://example.com:8080/path".to_string());
        assert!(url.is_ok());
        let url = url.unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port.unwrap(), 8080);
        assert_eq!(url.path, "/path");
    }

    #[test]
    fn test_url_host_port_path_query() {
        let url = Url::new("http://example.com:8080/path?a=123&b=456".to_string());
        assert!(url.is_ok());
        let url = url.unwrap();
        assert_eq!(url.host, "example.com");
        assert_eq!(url.port.unwrap(), 8080);
        assert_eq!(url.path, "/path");
        assert_eq!(url.query.unwrap(), "a=123&b=456");
    }
}
