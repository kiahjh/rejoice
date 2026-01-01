# Request Object

The `Req` type provides read-only access to incoming request data.

## Structure

```rust
pub struct Req {
    pub headers: HeaderMap,   // HTTP headers
    pub cookies: Cookies,     // Parsed cookies
    pub method: Method,       // HTTP method
    pub uri: Uri,             // Request URI
}
```

## Reading Headers

Access HTTP headers from the request:

```rust
use rejoice::{Req, Res, html};

pub async fn page(req: Req, res: Res) -> Res {
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
pub async fn page(req: Req, res: Res) -> Res {
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

## HTTP Method

Check the request method:

```rust
pub async fn page(req: Req, res: Res) -> Res {
    let method = req.method.as_str();
    
    match method {
        "GET" => { /* handle GET */ }
        "POST" => { /* handle POST */ }
        _ => { /* other methods */ }
    }
    
    res.html(html! {
        p { "Method: " (method) }
    })
}
```

## URI and Path

Access the request URI:

```rust
pub async fn page(req: Req, res: Res) -> Res {
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
pub async fn page(req: Req, res: Res) -> Res {
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
