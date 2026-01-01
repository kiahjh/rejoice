use axum::{
    extract::FromRequestParts,
    http::{request::Parts, HeaderMap, Method, Uri},
};
use std::collections::HashMap;

/// Incoming request data.
///
/// Provides read-only access to headers, cookies, method, and URI.
#[derive(Debug, Clone)]
pub struct Req {
    /// HTTP headers from the request
    pub headers: HeaderMap,
    /// Parsed cookies from the Cookie header
    pub cookies: Cookies,
    /// HTTP method (GET, POST, etc.)
    pub method: Method,
    /// Request URI
    pub uri: Uri,
}

/// A simple cookie jar for reading cookies from the request.
#[derive(Debug, Clone, Default)]
pub struct Cookies {
    cookies: HashMap<String, String>,
}

impl Cookies {
    /// Parse cookies from a Cookie header value
    pub fn from_header(header: Option<&str>) -> Self {
        let mut cookies = HashMap::new();

        if let Some(header) = header {
            for part in header.split(';') {
                let part = part.trim();
                if let Some((name, value)) = part.split_once('=') {
                    cookies.insert(name.trim().to_string(), value.trim().to_string());
                }
            }
        }

        Self { cookies }
    }

    /// Get a cookie value by name
    pub fn get(&self, name: &str) -> Option<&str> {
        self.cookies.get(name).map(|s| s.as_str())
    }

    /// Check if a cookie exists
    pub fn has(&self, name: &str) -> bool {
        self.cookies.contains_key(name)
    }

    /// Iterate over all cookies
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.cookies.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

impl<S> FromRequestParts<S> for Req
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let headers = parts.headers.clone();
        let cookies = Cookies::from_header(
            headers
                .get(axum::http::header::COOKIE)
                .and_then(|v| v.to_str().ok()),
        );
        let method = parts.method.clone();
        let uri = parts.uri.clone();

        Ok(Req {
            headers,
            cookies,
            method,
            uri,
        })
    }
}
