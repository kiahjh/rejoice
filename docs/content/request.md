# Request Object

The `Req` type provides access to incoming request data including headers, cookies, and body.

## Structure

```rust
pub struct Req {
    pub headers: HeaderMap,   // HTTP headers
    pub cookies: Cookies,     // Parsed cookies
    pub method: Method,       // HTTP method
    pub uri: Uri,             // Request URI
    pub body: Body,           // Request body
}
```

## Reading Headers

Access HTTP headers from the request:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    // Get a specific header
    let user_agent = req.headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");
    
    // Check for authorization
    let auth = req.headers.get("authorization");
    
    res.html(html! {
        p { "Your browser: " (user_agent) }
    })
}
```

## Reading Cookies

Access cookies from the request:

```rust
pub async fn get(req: Req, res: Res) -> Res {
    // Get a specific cookie
    let session = req.cookies.get("session_id");
    
    // Check if a cookie exists
    if req.cookies.has("remember_me") {
        // User wants to be remembered
    }
    
    // Iterate all cookies
    for (name, value) in req.cookies.iter() {
        println!("{}: {}", name, value);
    }
    
    match session {
        Some(sid) => res.html(html! {
            p { "Session: " (sid) }
        }),
        None => res.redirect("/login"),
    }
}
```

## Request Body

The `body` field provides methods for parsing POST/PUT/PATCH request data.

### Parsing Form Data

```rust
use rejoice::{Req, Res, html};
use serde::Deserialize;

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

pub async fn post(req: Req, res: Res) -> Res {
    let Ok(form) = req.body.as_form::<ContactForm>() else {
        return res.bad_request("Invalid form data");
    };
    
    // Use form.name, form.email, form.message...
    res.redirect("/thank-you")
}
```

### Parsing JSON

```rust
use rejoice::{Req, Res, json};
use serde::Deserialize;

#[derive(Deserialize)]
struct CreateUser {
    name: String,
    email: String,
}

pub async fn post(req: Req, res: Res) -> Res {
    let Ok(data) = req.body.as_json::<CreateUser>() else {
        return res.bad_request("Invalid JSON");
    };
    
    // Create user...
    res.json(&json!({ "id": 1, "name": data.name }))
}
```

### Body Methods

```rust
// Check if body is empty
req.body.is_empty()

// Get raw bytes
req.body.as_bytes()

// Parse as UTF-8 text
req.body.as_text() -> Result<String, BodyParseError>

// Parse as JSON
req.body.as_json::<T>() -> Result<T, BodyParseError>

// Parse as form data (application/x-www-form-urlencoded)
req.body.as_form::<T>() -> Result<T, BodyParseError>
```

## URI and Path

Access the request URI:

```rust
pub async fn get(req: Req, res: Res) -> Res {
    let path = req.uri.path();           // "/users/123"
    let query = req.uri.query();         // Some("page=2&sort=name")
    let full_uri = req.uri.to_string();  // "/users/123?page=2&sort=name"
    
    res.html(html! {
        p { "Path: " (path) }
        @if let Some(q) = query {
            p { "Query: " (q) }
        }
    })
}
```

## Authentication Pattern

Common pattern for checking authentication:

```rust
pub async fn get(req: Req, res: Res) -> Res {
    let session = req.cookies.get("session_id");
    
    let Some(session_id) = session else {
        return res.redirect("/login");
    };
    
    // User is authenticated, continue...
    res.html(html! {
        h1 { "Welcome back!" }
    })
}
```

## Next Steps

- [Response Object](/docs/response) - Build responses with cookies and headers
- [Layouts](/docs/layouts) - Use layouts for authentication checks
