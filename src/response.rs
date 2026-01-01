use axum::{
    body::Body,
    extract::FromRequestParts,
    http::{header, request::Parts, HeaderName, HeaderValue, StatusCode},
    response::IntoResponse,
};
use maud::Markup;
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap};

/// Response builder with interior mutability.
///
/// Use `set_*` methods to configure headers, cookies, and status,
/// then finalize with `html()`, `json()`, `redirect()`, or `raw()`.
#[derive(Debug)]
pub struct Res {
    inner: RefCell<ResInner>,
}

#[derive(Debug, Default)]
struct ResInner {
    status: Option<StatusCode>,
    headers: HashMap<String, String>,
    cookies: Vec<(String, String, CookieOptions)>,
    body: Option<ResBody>,
}

#[derive(Debug, Clone)]
struct CookieOptions {
    path: Option<String>,
    max_age: Option<i64>,
    http_only: bool,
    secure: bool,
    same_site: Option<String>,
}

impl Default for CookieOptions {
    fn default() -> Self {
        Self {
            path: Some("/".to_string()),
            max_age: None,
            http_only: true,
            secure: false,
            same_site: Some("Lax".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
enum ResBody {
    Html(String),
    Json(String),
    Redirect(String, bool), // (url, permanent)
    Raw(Vec<u8>),
}

impl Res {
    /// Create a new response builder
    pub fn new() -> Self {
        Self {
            inner: RefCell::new(ResInner::default()),
        }
    }

    /// Set a response header
    pub fn set_header(&self, name: impl Into<String>, value: impl Into<String>) -> &Self {
        self.inner
            .borrow_mut()
            .headers
            .insert(name.into(), value.into());
        self
    }

    /// Set the response status code
    pub fn set_status(&self, status: StatusCode) -> &Self {
        self.inner.borrow_mut().status = Some(status);
        self
    }

    /// Set a cookie on the response
    pub fn set_cookie(&self, name: impl Into<String>, value: impl Into<String>) -> &Self {
        self.inner.borrow_mut().cookies.push((
            name.into(),
            value.into(),
            CookieOptions::default(),
        ));
        self
    }

    /// Set a cookie with custom options.
    ///
    /// # Example
    /// ```ignore
    /// res.set_cookie_with_options("session", token, Some("/"), Some(3600), true, true, Some("Strict"))
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn set_cookie_with_options(
        &self,
        name: impl Into<String>,
        value: impl Into<String>,
        path: Option<&str>,
        max_age: Option<i64>,
        http_only: bool,
        secure: bool,
        same_site: Option<&str>,
    ) -> &Self {
        self.inner.borrow_mut().cookies.push((
            name.into(),
            value.into(),
            CookieOptions {
                path: path.map(|s| s.to_string()),
                max_age,
                http_only,
                secure,
                same_site: same_site.map(|s| s.to_string()),
            },
        ));
        self
    }

    /// Delete a cookie by setting it to expire immediately
    pub fn delete_cookie(&self, name: impl Into<String>) -> &Self {
        self.inner.borrow_mut().cookies.push((
            name.into(),
            String::new(),
            CookieOptions {
                path: Some("/".to_string()),
                max_age: Some(0),
                http_only: true,
                secure: false,
                same_site: Some("Lax".to_string()),
            },
        ));
        self
    }

    /// Finalize as an HTML response
    pub fn html(self, markup: Markup) -> Self {
        self.inner.borrow_mut().body = Some(ResBody::Html(markup.into_string()));
        if self.inner.borrow().status.is_none() {
            self.inner.borrow_mut().status = Some(StatusCode::OK);
        }
        self
    }

    /// Finalize as a JSON response
    pub fn json<T: Serialize>(self, data: &T) -> Self {
        let json_string = serde_json::to_string(data).unwrap_or_else(|_| "null".to_string());
        self.inner.borrow_mut().body = Some(ResBody::Json(json_string));
        if self.inner.borrow().status.is_none() {
            self.inner.borrow_mut().status = Some(StatusCode::OK);
        }
        self
    }

    /// Finalize as a redirect (302 Found)
    pub fn redirect(self, url: impl Into<String>) -> Self {
        self.inner.borrow_mut().body = Some(ResBody::Redirect(url.into(), false));
        self
    }

    /// Finalize as a permanent redirect (301 Moved Permanently)
    pub fn redirect_permanent(self, url: impl Into<String>) -> Self {
        self.inner.borrow_mut().body = Some(ResBody::Redirect(url.into(), true));
        self
    }

    /// Finalize as a raw byte response
    pub fn raw(self, body: impl Into<Vec<u8>>) -> Self {
        self.inner.borrow_mut().body = Some(ResBody::Raw(body.into()));
        if self.inner.borrow().status.is_none() {
            self.inner.borrow_mut().status = Some(StatusCode::OK);
        }
        self
    }

    /// Check if this response is HTML (for layout wrapping)
    pub fn is_html(&self) -> bool {
        matches!(self.inner.borrow().body, Some(ResBody::Html(_)))
    }

    /// Extract the HTML content for layout wrapping.
    /// Returns None if this is not an HTML response.
    pub fn take_html(&self) -> Option<String> {
        let mut inner = self.inner.borrow_mut();
        match &inner.body {
            Some(ResBody::Html(_)) => {
                if let Some(ResBody::Html(html)) = inner.body.take() {
                    Some(html)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// Set HTML content (used by layout wrapping)
    pub fn set_html(&self, html: String) {
        self.inner.borrow_mut().body = Some(ResBody::Html(html));
    }

    fn build_cookie_header(name: &str, value: &str, options: &CookieOptions) -> String {
        let mut cookie = format!("{}={}", name, value);

        if let Some(path) = &options.path {
            cookie.push_str(&format!("; Path={}", path));
        }
        if let Some(max_age) = options.max_age {
            cookie.push_str(&format!("; Max-Age={}", max_age));
        }
        if options.http_only {
            cookie.push_str("; HttpOnly");
        }
        if options.secure {
            cookie.push_str("; Secure");
        }
        if let Some(same_site) = &options.same_site {
            cookie.push_str(&format!("; SameSite={}", same_site));
        }

        cookie
    }
}

impl Default for Res {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for Res {
    fn clone(&self) -> Self {
        let inner = self.inner.borrow();
        Self {
            inner: RefCell::new(ResInner {
                status: inner.status,
                headers: inner.headers.clone(),
                cookies: inner.cookies.clone(),
                body: inner.body.clone(),
            }),
        }
    }
}

impl IntoResponse for Res {
    fn into_response(self) -> axum::response::Response {
        let inner = self.inner.into_inner();

        let (status, content_type, body) = match inner.body {
            Some(ResBody::Html(html)) => (
                inner.status.unwrap_or(StatusCode::OK),
                Some("text/html; charset=utf-8"),
                Body::from(html),
            ),
            Some(ResBody::Json(json)) => (
                inner.status.unwrap_or(StatusCode::OK),
                Some("application/json"),
                Body::from(json),
            ),
            Some(ResBody::Redirect(url, permanent)) => {
                let status = if permanent {
                    StatusCode::MOVED_PERMANENTLY
                } else {
                    StatusCode::FOUND
                };
                let mut response = axum::response::Response::builder()
                    .status(status)
                    .header(header::LOCATION, url)
                    .body(Body::empty())
                    .unwrap();

                // Add cookies to redirect response
                for (name, value, options) in inner.cookies {
                    let cookie_header = Self::build_cookie_header(&name, &value, &options);
                    response.headers_mut().append(
                        header::SET_COOKIE,
                        HeaderValue::from_str(&cookie_header).unwrap(),
                    );
                }

                // Add custom headers
                for (name, value) in inner.headers {
                    if let (Ok(name), Ok(value)) =
                        (name.parse::<HeaderName>(), value.parse::<HeaderValue>())
                    {
                        response.headers_mut().insert(name, value);
                    }
                }

                return response;
            }
            Some(ResBody::Raw(bytes)) => (
                inner.status.unwrap_or(StatusCode::OK),
                None,
                Body::from(bytes),
            ),
            None => (
                inner.status.unwrap_or(StatusCode::OK),
                None,
                Body::empty(),
            ),
        };

        let mut response = axum::response::Response::builder()
            .status(status)
            .body(body)
            .unwrap();

        // Set content type if we have one
        if let Some(ct) = content_type {
            response
                .headers_mut()
                .insert(header::CONTENT_TYPE, HeaderValue::from_static(ct));
        }

        // Add cookies
        for (name, value, options) in inner.cookies {
            let cookie_header = Self::build_cookie_header(&name, &value, &options);
            response.headers_mut().append(
                header::SET_COOKIE,
                HeaderValue::from_str(&cookie_header).unwrap(),
            );
        }

        // Add custom headers
        for (name, value) in inner.headers {
            if let (Ok(name), Ok(value)) =
                (name.parse::<HeaderName>(), value.parse::<HeaderValue>())
            {
                response.headers_mut().insert(name, value);
            }
        }

        response
    }
}

impl<S> FromRequestParts<S> for Res
where
    S: Send + Sync,
{
    type Rejection = std::convert::Infallible;

    async fn from_request_parts(_parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        Ok(Res::new())
    }
}
