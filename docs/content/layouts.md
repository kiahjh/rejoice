# Layouts

Layouts wrap your pages with shared UI like navigation, headers, and footers. They nest automatically based on directory structure.

## Basic Layout

Create a `layout.rs` file in any routes directory:

```rust
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    res.html(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "My App" }
            }
            body {
                nav {
                    a href="/" { "Home" }
                    a href="/about" { "About" }
                }
                main { (children) }
                footer { "Built with Rejoice" }
            }
        }
    })
}
```

The `children` parameter contains the rendered page content.

## Root Layout

A layout at `src/routes/layout.rs` wraps **all** pages in your app. This is where you typically put:

- The HTML document structure (`<!DOCTYPE html>`, `<html>`, `<head>`, `<body>`)
- Global navigation
- Meta tags
- Global styles

## Nested Layouts

Layouts nest based on directory structure:

```text
src/routes/
├── layout.rs           # Root layout (wraps everything)
├── index.rs            # /
├── about.rs            # /about
└── dashboard/
    ├── layout.rs       # Dashboard layout (wraps /dashboard/*)
    ├── index.rs        # /dashboard
    └── settings.rs     # /dashboard/settings
```

For `/dashboard/settings`:
1. `settings.rs` renders the page content
2. `dashboard/layout.rs` wraps it with dashboard UI
3. `layout.rs` wraps everything with the document structure

## Layout with State

If your app uses state, layouts receive it as the first parameter:

```rust
use crate::AppState;
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    // Access state.db, state.config, etc.
    res.html(html! {
        (DOCTYPE)
        html {
            head { title { "My App" } }
            body { (children) }
        }
    })
}
```

## Layout Bypass

Non-HTML responses automatically bypass layouts. This is useful for:

- Redirects
- API endpoints returning JSON
- File downloads

```rust
pub async fn page(req: Req, res: Res) -> Res {
    let session = req.cookies.get("session_id");
    
    if session.is_none() {
        // This redirect bypasses all layouts
        return res.redirect("/login");
    }
    
    // This HTML gets wrapped in layouts
    res.html(html! {
        h1 { "Dashboard" }
    })
}
```

## Authentication Pattern

Use layouts to protect groups of routes:

```rust
// src/routes/admin/layout.rs
use crate::AppState;
use rejoice::{Children, Req, Res, html};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    let session = req.cookies.get("session_id");
    
    // Check authentication
    if !is_admin(&state, session).await {
        return res.redirect("/login");
    }
    
    // Render admin layout
    res.html(html! {
        div class="admin-layout" {
            aside { /* Admin sidebar */ }
            main { (children) }
        }
    })
}
```

All pages under `src/routes/admin/` will be protected.

## Next Steps

- [HTML Templates](/docs/templates) - Learn Maud syntax
- [Request Object](/docs/request) - Read cookies and headers
- [Response Object](/docs/response) - Redirects and cookies
