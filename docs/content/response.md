# Response Object

The `Res` type is a response builder that uses interior mutability for flexible response construction.

## Response Flow

1. Use `set_*` methods to configure the response (headers, cookies, status)
2. Call a finalizer method to return the response

## Finalizer Methods

### HTML Response

Return server-rendered HTML:

```rust
use rejoice::{Req, Res, html};

pub async fn page(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}
```

Returns: `200 OK` with `Content-Type: text/html; charset=utf-8`

### JSON Response

Return JSON data:

```rust
use rejoice::{Req, Res, json};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

pub async fn page(req: Req, res: Res) -> Res {
    let user = User { id: 1, name: "Alice".into() };
    res.json(&user)
}

// Or with the json! macro:
pub async fn api(req: Req, res: Res) -> Res {
    res.json(&json!({
        "status": "ok",
        "count": 42
    }))
}
```

Returns: `200 OK` with `Content-Type: application/json`

### Redirect

Redirect to another URL:

```rust
pub async fn page(req: Req, res: Res) -> Res {
    // Temporary redirect (302 Found)
    res.redirect("/login")
}

pub async fn moved(req: Req, res: Res) -> Res {
    // Permanent redirect (301 Moved Permanently)
    res.redirect_permanent("/new-url")
}
```

### Raw Response

Return raw bytes with custom content type:

```rust
pub async fn download(req: Req, res: Res) -> Res {
    let pdf_bytes = get_pdf_data();
    
    res.set_header("Content-Type", "application/pdf")
       .set_header("Content-Disposition", "attachment; filename=\"report.pdf\"")
       .raw(pdf_bytes)
}
```

## Setting Headers

Add custom headers to the response:

```rust
pub async fn page(req: Req, res: Res) -> Res {
    res.set_header("X-Custom-Header", "value")
       .set_header("Cache-Control", "max-age=3600")
       .html(html! { h1 { "Hello!" } })
}
```

## Setting Status Code

Override the default status code:

```rust
use axum::http::StatusCode;

pub async fn not_found(req: Req, res: Res) -> Res {
    res.set_status(StatusCode::NOT_FOUND)
       .html(html! { h1 { "Page not found" } })
}

pub async fn created(req: Req, res: Res) -> Res {
    res.set_status(StatusCode::CREATED)
       .json(&json!({ "id": 123 }))
}
```

## Setting Cookies

### Simple Cookie

```rust
pub async fn page(req: Req, res: Res) -> Res {
    res.set_cookie("visited", "true")
       .html(html! { h1 { "Welcome!" } })
}
```

### Cookie with Options

```rust
pub async fn login(req: Req, res: Res) -> Res {
    res.set_cookie_with_options(
        "session_id",           // name
        "abc123",               // value
        Some("/"),              // path
        Some(3600),             // max_age (seconds)
        true,                   // http_only
        true,                   // secure
        Some("Strict"),         // same_site
    )
    .redirect("/dashboard")
}
```

### Delete Cookie

```rust
pub async fn logout(req: Req, res: Res) -> Res {
    res.delete_cookie("session_id")
       .redirect("/")
}
```

## Chaining Methods

All `set_*` methods return `&Res` and can be chained:

```rust
pub async fn page(req: Req, res: Res) -> Res {
    res.set_cookie("last_visit", "2025-01-01")
       .set_header("X-Frame-Options", "DENY")
       .set_header("Cache-Control", "no-cache")
       .html(html! { h1 { "Secured page" } })
}
```

## Cookies Persist Across Branches

Cookies set on `res` apply to all subsequent responses:

```rust
pub async fn page(req: Req, res: Res) -> Res {
    // This cookie is set regardless of which branch executes
    res.set_cookie("last_visit", "2025-01-01");
    
    if !is_authenticated(&req) {
        // Redirect gets the "last_visit" cookie
        return res.redirect("/login");
    }
    
    // HTML also gets the "last_visit" cookie
    res.html(html! { h1 { "Dashboard" } })
}
```

## Next Steps

- [Request Object](/docs/request) - Read incoming request data
- [Layouts](/docs/layouts) - Redirects bypass layouts
- [Database](/docs/database) - Query data for responses
