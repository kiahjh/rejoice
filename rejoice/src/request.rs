use axum::{
    body::Bytes,
    extract::FromRequest,
    http::{HeaderMap, Method, Request, Uri},
};
use serde::de::DeserializeOwned;
use std::collections::HashMap;

/// Incoming request data.
///
/// Provides read-only access to headers, cookies, method, URI, and body.
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
    /// Request body (for POST, PUT, etc.)
    pub body: Body,
}

/// Request body with parsing methods.
#[derive(Debug, Clone, Default)]
pub struct Body {
    bytes: Bytes,
}

/// Error type for body parsing failures.
#[derive(Debug)]
pub struct BodyParseError {
    pub message: String,
}

impl std::fmt::Display for BodyParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for BodyParseError {}

impl Body {
    /// Create a new Body from bytes
    pub fn new(bytes: Bytes) -> Self {
        Self { bytes }
    }

    /// Check if the body is empty
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Get the raw bytes of the body
    pub fn as_bytes(&self) -> &Bytes {
        &self.bytes
    }

    /// Parse the body as UTF-8 text
    pub fn as_text(&self) -> Result<String, BodyParseError> {
        String::from_utf8(self.bytes.to_vec()).map_err(|e| BodyParseError {
            message: format!("Invalid UTF-8: {}", e),
        })
    }

    /// Parse the body as JSON into a typed struct
    ///
    /// # Example
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct LoginData {
    ///     email: String,
    ///     password: String,
    /// }
    ///
    /// pub async fn post(req: Req, res: Res) -> Res {
    ///     let Ok(data) = req.body.as_json::<LoginData>() else {
    ///         return res.bad_request("Invalid JSON");
    ///     };
    ///     // use data.email, data.password...
    /// }
    /// ```
    pub fn as_json<T: DeserializeOwned>(&self) -> Result<T, BodyParseError> {
        serde_json::from_slice(&self.bytes).map_err(|e| BodyParseError {
            message: format!("Invalid JSON: {}", e),
        })
    }

    /// Parse the body as form data (application/x-www-form-urlencoded)
    ///
    /// # Example
    /// ```ignore
    /// #[derive(Deserialize)]
    /// struct ContactForm {
    ///     name: String,
    ///     email: String,
    ///     message: String,
    /// }
    ///
    /// pub async fn post(req: Req, res: Res) -> Res {
    ///     let Ok(form) = req.body.as_form::<ContactForm>() else {
    ///         return res.bad_request("Invalid form data");
    ///     };
    ///     // use form.name, form.email, form.message...
    /// }
    /// ```
    pub fn as_form<T: DeserializeOwned>(&self) -> Result<T, BodyParseError> {
        serde_urlencoded::from_bytes(&self.bytes).map_err(|e| BodyParseError {
            message: format!("Invalid form data: {}", e),
        })
    }
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

impl<S> FromRequest<S> for Req
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request(req: Request<axum::body::Body>, _state: &S) -> Result<Self, Self::Rejection> {
        let (parts, body) = req.into_parts();
        
        let headers = parts.headers;
        let cookies = Cookies::from_header(
            headers
                .get(axum::http::header::COOKIE)
                .and_then(|v| v.to_str().ok()),
        );
        let method = parts.method;
        let uri = parts.uri;
        
        // Read the body bytes
        let bytes = axum::body::to_bytes(body, usize::MAX)
            .await
            .unwrap_or_default();

        Ok(Req {
            headers,
            cookies,
            method,
            uri,
            body: Body::new(bytes),
        })
    }
}
