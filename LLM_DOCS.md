# Rejoice Framework - Complete Documentation for AI Agents

This document provides exhaustive documentation for building web applications with the Rejoice framework. It is designed for AI coding agents to have complete knowledge of all Rejoice features and patterns.

## Table of Contents

1. [Overview](#overview)
2. [Project Structure](#project-structure)
3. [CLI Commands](#cli-commands)
4. [Routes and Pages](#routes-and-pages)
5. [Layouts](#layouts)
6. [Request Object (Req)](#request-object-req)
7. [Response Object (Res)](#response-object-res)
8. [HTML Templating with Maud](#html-templating-with-maud)
9. [Application State](#application-state)
10. [Database Support](#database-support)
11. [Dynamic Routes](#dynamic-routes)
12. [SolidJS Islands](#solidjs-islands)
13. [Tailwind CSS](#tailwind-css)
14. [Static Assets](#static-assets)
15. [Environment Variables](#environment-variables)
16. [Imports and Exports](#imports-and-exports)
17. [Complete Examples](#complete-examples)

---

## Overview

Rejoice is a Rust web framework with:
- File-based routing
- Nested layouts
- Live reload during development
- Tailwind CSS v4 integration
- SolidJS islands for client-side interactivity
- Optional SQLite database support
- Type-safe HTML templating with Maud

---

## Project Structure

A Rejoice project has the following structure:

```
my-app/
├── build.rs                 # Build script for route generation
├── Cargo.toml               # Rust dependencies
├── package.json             # Node.js dependencies (Vite, Tailwind, SolidJS)
├── vite.config.ts           # Vite configuration
├── tsconfig.json            # TypeScript configuration
├── .env                     # Environment variables (if using database)
├── .gitignore
├── client/                  # Client-side code
│   ├── styles.css           # Tailwind CSS entry point
│   ├── Counter.tsx          # Example SolidJS component
│   └── islands.tsx          # Auto-generated islands registry (do not edit)
├── public/                  # Static assets served at root URL
│   └── (images, fonts, etc.)
├── dist/                    # Built client assets (auto-generated)
│   ├── islands.js
│   └── styles.css
└── src/
    ├── main.rs              # Application entry point
    ├── routes.rs            # Auto-generated route modules (do not edit)
    └── routes/              # File-based routes
        ├── layout.rs        # Root layout (wraps all pages)
        ├── index.rs         # GET /
        ├── about.rs         # GET /about
        └── users/
            ├── layout.rs    # Layout for /users/* routes
            ├── index.rs     # GET /users
            └── [id].rs      # GET /users/:id (dynamic route)
```

---

## CLI Commands

### `rejoice init [name] [--with-db]`

Creates a new Rejoice project.

```bash
# Create a new project in a new directory
rejoice init my-app

# Create a project with SQLite database support
rejoice init my-app --with-db

# Initialize in current directory (uses directory name as project name)
cd my-empty-dir
rejoice init
```

### `rejoice dev`

Starts the development server with:
- Automatic Rust recompilation on file changes
- Vite watch mode for client assets
- WebSocket-based live reload
- Hot module replacement for Rust and client changes

```bash
rejoice dev
# Server runs at http://localhost:8080
# Live reload WebSocket at ws://localhost:3001/__reload
```

### `rejoice build [--release]`

Builds the project for deployment.

```bash
# Development build
rejoice build

# Production build with optimizations
rejoice build --release
```

Build steps:
1. Install npm dependencies (if `node_modules/` doesn't exist)
2. Generate islands registry (if islands exist in `client/`)
3. Build client assets with Vite
4. Compile Rust binary

Output:
- Binary: `target/debug/<name>` or `target/release/<name>`
- Client assets: `dist/islands.js`, `dist/styles.css`

---

## Routes and Pages

Routes are defined by creating `.rs` files in `src/routes/`. Each route file exports functions named after HTTP methods: `get`, `post`, `put`, `delete`, `patch`.

### File-to-URL Mapping

| File Path | URL Path |
|-----------|----------|
| `src/routes/index.rs` | `/` |
| `src/routes/about.rs` | `/about` |
| `src/routes/contact.rs` | `/contact` |
| `src/routes/users/index.rs` | `/users` |
| `src/routes/users/profile.rs` | `/users/profile` |
| `src/routes/users/[id].rs` | `/users/:id` (dynamic) |
| `src/routes/blog/[slug].rs` | `/blog/:slug` (dynamic) |

### HTTP Method Functions

Each route file can export one or more HTTP method handlers:

```rust
use rejoice::{Req, Res, html};

// GET request handler
pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}

// POST request handler
pub async fn post(req: Req, res: Res) -> Res {
    // Handle form submission, API call, etc.
    res.redirect("/success")
}
```

### Stateless Route (No Application State)

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}
```

### Stateful Route (With Application State)

```rust
use crate::AppState;
use rejoice::{Req, Res, html};

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Access state.db, state.config, etc.
    res.html(html! {
        h1 { "Hello from stateful route!" }
    })
}
```

### Route Function Signature

**Stateless apps** (using `routes!()`):
```rust
pub async fn get(req: Req, res: Res) -> Res
pub async fn post(req: Req, res: Res) -> Res
```

**Stateful apps** (using `routes!(AppState)`):
```rust
pub async fn get(state: AppState, req: Req, res: Res) -> Res
pub async fn post(state: AppState, req: Req, res: Res) -> Res
```

**Dynamic routes** add a path parameter:
```rust
// Stateless
pub async fn get(req: Req, res: Res, id: String) -> Res

// Stateful
pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res
```

---

## Layouts

Layouts wrap pages and provide shared UI. Create a `layout.rs` file in any routes directory. Layouts export a `layout` function (not an HTTP method name).

### Root Layout

`src/routes/layout.rs` wraps ALL pages:

```rust
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    res.html(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                meta name="viewport" content="width=device-width, initial-scale=1";
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

### Nested Layouts

Layouts nest automatically. Given this structure:

```
src/routes/
├── layout.rs           # Root layout
├── index.rs            # /
└── admin/
    ├── layout.rs       # Admin layout
    ├── index.rs        # /admin
    └── users.rs        # /admin/users
```

The `/admin/users` page is wrapped:
1. First by `routes/admin/layout.rs`
2. Then by `routes/layout.rs` (root)

### Layout with State

```rust
use crate::AppState;
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    // Access state for shared data (e.g., user info from session)
    res.html(html! {
        (DOCTYPE)
        html {
            head { title { "My App" } }
            body { (children) }
        }
    })
}
```

### Layout Function Signature

**Stateless apps**:
```rust
pub async fn layout(req: Req, res: Res, children: Children) -> Res
```

**Stateful apps**:
```rust
pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res
```

### Layout Bypass

Non-HTML responses (redirects, JSON, raw) bypass layout wrapping automatically:

```rust
pub async fn get(req: Req, res: Res) -> Res {
    if !is_authenticated(&req) {
        // This redirect is NOT wrapped in layouts
        return res.redirect("/login");
    }
    
    // This HTML IS wrapped in layouts
    res.html(html! {
        h1 { "Dashboard" }
    })
}
```

---

## Request Object (Req)

The `Req` type provides access to incoming request data including headers, cookies, method, URI, and body.

### Fields

```rust
pub struct Req {
    pub headers: HeaderMap,   // HTTP headers
    pub cookies: Cookies,     // Parsed cookies
    pub method: Method,       // HTTP method (GET, POST, etc.)
    pub uri: Uri,             // Request URI
    pub body: Body,           // Request body (for POST, PUT, etc.)
}
```

### Reading Headers

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    // Get a specific header
    let user_agent = req.headers
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("Unknown");
    
    let auth = req.headers.get("authorization");
    
    res.html(html! {
        p { "User Agent: " (user_agent) }
    })
}
```

### Reading Cookies

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    // Get a specific cookie
    let session = req.cookies.get("session_id");
    
    // Check if a cookie exists
    if req.cookies.has("remember_me") {
        // ...
    }
    
    // Iterate all cookies
    for (name, value) in req.cookies.iter() {
        println!("{}: {}", name, value);
    }
    
    match session {
        Some(sid) => res.html(html! { p { "Session: " (sid) } }),
        None => res.redirect("/login"),
    }
}
```

### Reading Method and URI

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    let method = req.method.as_str();  // "GET", "POST", etc.
    let path = req.uri.path();          // "/users/123"
    let query = req.uri.query();        // Some("page=2&sort=name")
    
    res.html(html! {
        p { "Method: " (method) }
        p { "Path: " (path) }
    })
}
```

### Request Body

The `Body` type provides methods for parsing request body data (useful for POST, PUT, PATCH requests).

#### Parsing JSON

```rust
use rejoice::{Req, Res, html};
use serde::Deserialize;

#[derive(Deserialize)]
struct LoginData {
    email: String,
    password: String,
}

pub async fn post(req: Req, res: Res) -> Res {
    let Ok(data) = req.body.as_json::<LoginData>() else {
        return res.bad_request("Invalid JSON");
    };
    
    // Use data.email, data.password...
    res.redirect("/dashboard")
}
```

#### Parsing Form Data

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

#### Body Methods

```rust
// Check if body is empty
req.body.is_empty()

// Get raw bytes
req.body.as_bytes()

// Parse as UTF-8 text
req.body.as_text() -> Result<String, BodyParseError>

// Parse as JSON into typed struct
req.body.as_json::<T>() -> Result<T, BodyParseError>

// Parse as form data (application/x-www-form-urlencoded)
req.body.as_form::<T>() -> Result<T, BodyParseError>
```
```

---

## Response Object (Res)

The `Res` type is a response builder with interior mutability. Use `set_*` methods to configure the response, then finalize with a response method.

### Mutator Methods (Chainable)

All mutator methods return `&Res` for optional chaining:

```rust
// Set a response header
res.set_header("X-Custom-Header", "value");

// Set the HTTP status code
res.set_status(StatusCode::CREATED);

// Set a cookie (with sensible defaults)
res.set_cookie("session_id", "abc123");

// Set a cookie with custom options
res.set_cookie_with_options(
    "session_id",           // name
    "abc123",               // value
    Some("/"),              // path
    Some(3600),             // max_age in seconds
    true,                   // http_only
    true,                   // secure
    Some("Strict"),         // same_site: "Strict", "Lax", or "None"
);

// Delete a cookie
res.delete_cookie("session_id");
```

### Finalizer Methods

Finalizers consume the `Res` and return the final response:

#### HTML Response

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}
```

Returns: `200 OK` with `Content-Type: text/html; charset=utf-8`

#### JSON Response

```rust
use rejoice::{Req, Res, json};
use serde::Serialize;

#[derive(Serialize)]
struct User {
    id: i32,
    name: String,
}

pub async fn get(req: Req, res: Res) -> Res {
    let user = User { id: 1, name: "Alice".into() };
    res.json(&user)
}

// Or with the json! macro:
pub async fn get(req: Req, res: Res) -> Res {
    res.json(&json!({
        "id": 1,
        "name": "Alice",
        "roles": ["admin", "user"]
    }))
}
```

Returns: `200 OK` with `Content-Type: application/json`

#### Redirect

```rust
use rejoice::{Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    // Temporary redirect (302 Found)
    res.redirect("/login")
}

pub async fn get(req: Req, res: Res) -> Res {
    // Permanent redirect (301 Moved Permanently)
    res.redirect_permanent("/new-url")
}
```

#### Raw Response

```rust
use rejoice::{Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let pdf_bytes = get_pdf_data();
    
    res.set_header("Content-Type", "application/pdf")
       .set_header("Content-Disposition", "attachment; filename=\"report.pdf\"")
       .raw(pdf_bytes)
}
```

### Error Response Helpers

Convenient methods for common HTTP error responses:

```rust
use rejoice::{Req, Res};

pub async fn post(req: Req, res: Res) -> Res {
    // 400 Bad Request
    res.bad_request("Invalid form data")
}

pub async fn get(req: Req, res: Res) -> Res {
    // 401 Unauthorized
    res.unauthorized("Please log in")
}

pub async fn get(req: Req, res: Res) -> Res {
    // 403 Forbidden
    res.forbidden("You don't have access")
}

pub async fn get(req: Req, res: Res) -> Res {
    // 404 Not Found
    res.not_found("Page not found")
}

pub async fn get(req: Req, res: Res) -> Res {
    // 500 Internal Server Error
    res.internal_error("Something went wrong")
}
```

Each error helper returns an HTML response with the status code and message.
```

### Chaining Example

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.set_cookie("visited", "true")
       .set_header("X-Frame-Options", "DENY")
       .set_header("Cache-Control", "no-cache")
       .html(html! {
           h1 { "Welcome!" }
       })
}
```

### Setting Cookies Before Branching

Cookies set on `res` apply to ALL subsequent responses:

```rust
pub async fn get(req: Req, res: Res) -> Res {
    // This cookie is set regardless of which branch executes
    res.set_cookie("last_visit", "2025-01-01");
    
    if !is_authenticated(&req) {
        // Redirect gets the "last_visit" cookie
        return res.redirect("/login");
    }
    
    // HTML response also gets the "last_visit" cookie
    res.html(html! { h1 { "Dashboard" } })
}
```

---

## HTML Templating with Maud

Rejoice uses [Maud](https://maud.lambda.xyz/) for compile-time HTML templating.

### Basic Syntax

```rust
use rejoice::{html, Markup, DOCTYPE};

fn render_page() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "My Page" }
            }
            body {
                h1 { "Hello!" }
                p { "Welcome to my site." }
            }
        }
    }
}
```

### Attributes

```rust
html! {
    a href="https://example.com" target="_blank" { "Link" }
    img src="/logo.png" alt="Logo";
    input type="text" name="email" placeholder="Enter email";
    div class="container mx-auto" id="main" { "Content" }
}
```

### Boolean Attributes

```rust
html! {
    input type="checkbox" checked;
    input type="text" disabled;
    button disabled[is_loading] { "Submit" }
}
```

### Dynamic Content

```rust
let name = "Alice";
let count = 42;

html! {
    p { "Hello, " (name) "!" }
    p { "Count: " (count) }
    p { "Calculated: " (count * 2) }
}
```

### Conditionals

```rust
let is_admin = true;
let user: Option<&str> = Some("Alice");

html! {
    @if is_admin {
        p { "Admin panel" }
    } @else {
        p { "User view" }
    }
    
    @if let Some(name) = user {
        p { "Welcome, " (name) }
    }
}
```

### Loops

```rust
let items = vec!["Apple", "Banana", "Cherry"];

html! {
    ul {
        @for item in &items {
            li { (item) }
        }
    }
}

// With index
html! {
    ol {
        @for (i, item) in items.iter().enumerate() {
            li { (i + 1) ". " (item) }
        }
    }
}
```

### Raw HTML (PreEscaped)

```rust
use rejoice::PreEscaped;

let raw_html = "<strong>Bold</strong>";

html! {
    // This escapes HTML entities (safe)
    p { (raw_html) }
    // Output: <p>&lt;strong&gt;Bold&lt;/strong&gt;</p>
    
    // This renders raw HTML (use with caution!)
    p { (PreEscaped(raw_html)) }
    // Output: <p><strong>Bold</strong></p>
}
```

### Components (Functions)

```rust
use rejoice::{html, Markup};

fn card(title: &str, content: &str) -> Markup {
    html! {
        div class="card" {
            h2 { (title) }
            p { (content) }
        }
    }
}

fn page() -> Markup {
    html! {
        (card("Welcome", "Hello, world!"))
        (card("About", "Learn more about us."))
    }
}
```

### DOCTYPE

Always include `DOCTYPE` for HTML5 documents:

```rust
use rejoice::{html, DOCTYPE};

html! {
    (DOCTYPE)
    html {
        head { title { "My App" } }
        body { /* ... */ }
    }
}
```

---

## Application State

### Stateless Applications

For simple apps without shared state:

**`src/main.rs`**:
```rust
use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
```

**Route files**:
```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! { h1 { "Hello!" } })
}
```

### Stateful Applications

For apps with shared state (database, config, services):

**`src/main.rs`**:
```rust
use rejoice::App;

rejoice::routes!(AppState);

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub config: AppConfig,
}

#[tokio::main]
async fn main() {
    let state = AppState {
        db: create_pool(/* ... */).await,
        config: load_config(),
    };
    
    let app = App::with_state(8080, create_router(), state);
    app.run().await;
}
```

**Route files**:
```rust
use crate::AppState;
use rejoice::{Req, Res, html};

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Use state.db, state.config, etc.
    res.html(html! { h1 { "Hello!" } })
}
```

### State Requirements

Your state type must implement:
- `Clone`
- `Send`
- `Sync`
- `'static`

Most types satisfy these automatically. Use `Arc` for non-Clone types:

```rust
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub expensive_resource: Arc<ExpensiveResource>,
}
```

---

## Database Support

Database support requires the `sqlite` feature flag.

### Setup with CLI

```bash
rejoice init my-app --with-db
```

This creates:
- `<project>.db` - Empty SQLite database file
- `.env` - Contains `DATABASE_URL`
- `AppState` with connection pool

### Manual Setup

**`Cargo.toml`**:
```toml
[dependencies]
rejoice = { version = "0.10.0", features = ["sqlite"] }
```

**`src/main.rs`**:
```rust
use std::time::Duration;
use rejoice::{
    App,
    db::{Pool, PoolConfig, Sqlite, create_pool},
};

rejoice::routes!(AppState);

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let pool = create_pool(PoolConfig {
        db_url: rejoice::env!("DATABASE_URL").to_string(),
        max_connections: 5,
        acquire_timeout: Duration::from_secs(3),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
    }).await;

    let state = AppState { db: pool };
    let app = App::with_state(8080, create_router(), state);
    app.run().await;
}
```

### Database Queries

```rust
use crate::AppState;
use rejoice::{Req, Res, html, db::{query, query_as, FromRow}};

#[derive(FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Query with query_as (returns typed results)
    let users: Vec<User> = query_as("SELECT id, name, email FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    
    // Query with parameters
    let user: Option<User> = query_as("SELECT * FROM users WHERE id = ?")
        .bind(123)
        .fetch_optional(&state.db)
        .await
        .unwrap();
    
    // Insert/Update/Delete
    query("INSERT INTO users (name, email) VALUES (?, ?)")
        .bind("Alice")
        .bind("alice@example.com")
        .execute(&state.db)
        .await
        .unwrap();
    
    res.html(html! {
        h1 { "Users" }
        ul {
            @for user in &users {
                li { (user.name) " - " (user.email) }
            }
        }
    })
}
```

### Database Exports

The `rejoice::db` module exports:
- `Pool<Sqlite>` - Connection pool type
- `Sqlite` - Database driver type
- `query` - Raw query function
- `query_as` - Typed query function
- `FromRow` - Derive macro for mapping query results to structs
- `PoolConfig` - Pool configuration struct
- `create_pool` - Pool creation function

---

## Dynamic Routes

Dynamic routes use square brackets in the filename: `[param].rs`

### Basic Dynamic Route

**`src/routes/users/[id].rs`** → `/users/:id`

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res, id: String) -> Res {
    res.html(html! {
        h1 { "User ID: " (id) }
    })
}
```

### With State

```rust
use crate::AppState;
use rejoice::{Req, Res, html, db::query_as};

pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res {
    let user_id: i32 = id.parse().unwrap_or(0);
    
    let user = query_as::<_, User>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .unwrap();
    
    match user {
        Some(u) => res.html(html! {
            h1 { "User: " (u.name) }
        }),
        None => res.not_found("User not found"),
    }
}
```

### Multiple Dynamic Segments

**`src/routes/orgs/[org_id]/repos/[repo_id].rs`** → `/orgs/:org_id/repos/:repo_id`

Note: Currently only single dynamic segments are fully supported per route file.

---

## SolidJS Islands

Islands are interactive SolidJS components embedded in server-rendered HTML.

### How Islands Work

1. Create a TSX component in `client/`
2. Use the `island!` macro in Rust to place it
3. Rejoice renders a placeholder `<div data-island="...">`
4. Client-side JavaScript hydrates the component

### Creating an Island Component

**`client/Counter.tsx`**:
```tsx
import { createSignal } from "solid-js";

interface CounterProps {
  initial: number;
  label?: string;
}

export default function Counter(props: CounterProps) {
  const [count, setCount] = createSignal(props.initial);

  return (
    <div>
      <span>{props.label || "Count"}: {count()}</span>
      <button onClick={() => setCount(c => c + 1)}>+</button>
      <button onClick={() => setCount(c => c - 1)}>-</button>
    </div>
  );
}
```

### Using an Island in Rust

```rust
use rejoice::{Req, Res, html, island};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Interactive Counter" }
        
        // Island with props
        (island!(Counter, { initial: 0 }))
        
        // Island with multiple props
        (island!(Counter, { initial: 10, label: "Score" }))
        
        // Island without props
        (island!(SimpleComponent))
    })
}
```

### Island Props Syntax

```rust
// No props
(island!(ComponentName))

// Single prop
(island!(ComponentName, { value: 42 }))

// Multiple props
(island!(ComponentName, { 
    count: 0,
    name: "Alice",
    enabled: true
}))

// Dynamic values
let user_id = 123;
(island!(UserCard, { id: user_id }))
```

### Component File Naming

- Files must be in `client/` directory
- Use `.tsx` or `.jsx` extension
- Component name matches filename (e.g., `Counter.tsx` → `Counter`)
- Export the component as `default`
- `islands.tsx` is auto-generated (do not edit)
- `styles.css` is for Tailwind (not a component)

### Islands Registry

The file `client/islands.tsx` is automatically generated during `rejoice dev` and `rejoice build`. It:
- Imports all component files
- Creates a registry mapping names to components
- Handles hydration when the page loads
- Exposes `window.__hydrateIslands()` for live reload

---

## Tailwind CSS

Tailwind CSS v4 is included and configured automatically.

### Using Tailwind Classes

In Rust templates:
```rust
pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        div class="container mx-auto px-4" {
            h1 class="text-4xl font-bold text-blue-600" {
                "Welcome!"
            }
            p class="mt-4 text-gray-700 leading-relaxed" {
                "This is styled with Tailwind."
            }
            button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600" {
                "Click me"
            }
        }
    })
}
```

In TSX components:
```tsx
export default function Card(props: { title: string }) {
  return (
    <div class="bg-white shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold">{props.title}</h2>
    </div>
  );
}
```

### Tailwind Configuration

**`client/styles.css`**:
```css
@import "tailwindcss";

@source "../src/**/*.rs";
@source "./**/*.tsx";
```

The `@source` directives tell Tailwind to scan:
- All Rust files in `src/` for class names
- All TSX files in `client/` for class names

### Custom CSS

Add custom styles in `client/styles.css`:

```css
@import "tailwindcss";

@source "../src/**/*.rs";
@source "./**/*.tsx";

/* Custom styles */
.custom-gradient {
  background: linear-gradient(to right, #3b82f6, #8b5cf6);
}
```

---

## Static Assets

The `public/` directory serves static files at the root URL.

### File Mapping

| File Path | URL |
|-----------|-----|
| `public/logo.png` | `/logo.png` |
| `public/images/hero.jpg` | `/images/hero.jpg` |
| `public/favicon.ico` | `/favicon.ico` |
| `public/fonts/custom.woff2` | `/fonts/custom.woff2` |

### Usage in Templates

```rust
html! {
    img src="/logo.png" alt="Logo";
    link rel="icon" href="/favicon.ico";
    link rel="stylesheet" href="/fonts/fonts.css";
}
```

### Built Assets

Client assets (JS, CSS) are built to `dist/` and served at `/static/`:
- `/static/islands.js` - Bundled island components
- `/static/styles.css` - Compiled Tailwind CSS

These are injected automatically into HTML responses.

---

## Environment Variables

### Compile-Time Variables

Use `rejoice::env!` to read from `.env` at compile time:

```rust
let db_url = rejoice::env!("DATABASE_URL");
let api_key = rejoice::env!("API_KEY");
```

This uses `dotenvy_macro` under the hood, so:
- Variables must exist in `.env` at build time
- Values are embedded in the binary
- Changes require recompilation

### Runtime Variables

For runtime configuration, use `std::env`:

```rust
let port = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap();

let app = App::new(port, create_router());
```

---

## Imports and Exports

### Root Level Exports

```rust
use rejoice::{
    // Core types
    App,             // Server/application struct
    Req,             // Request data
    Res,             // Response builder
    Children,        // Layout children type (alias for Markup)
    Path,            // Axum path extractor for dynamic routes
    
    // HTML templating (from Maud)
    html,            // HTML macro
    Markup,          // HTML type
    DOCTYPE,         // HTML5 doctype
    PreEscaped,      // Raw HTML wrapper
    
    // JSON
    json,            // serde_json::json! macro
    
    // Islands
    island,          // Island placement macro
};
```

### Prelude Module

For convenience, import everything common at once:

```rust
use rejoice::prelude::*;
// Imports: App, Req, Res, Children, Path, html, Markup, DOCTYPE, PreEscaped, json, island
```

### Database Module (Feature-Gated)

Requires `features = ["sqlite"]`:

```rust
use rejoice::db::{
    Pool,            // sqlx Pool type
    Sqlite,          // SQLite driver type
    query,           // Raw SQL query
    query_as,        // Typed SQL query
    FromRow,         // Derive macro for result mapping
    PoolConfig,      // Pool configuration
    create_pool,     // Pool creation function
};
```

### Main.rs Setup

**Stateless app**:
```rust
use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
```

**Stateful app**:
```rust
use rejoice::App;

rejoice::routes!(AppState);

#[derive(Clone)]
pub struct AppState { /* ... */ }

#[tokio::main]
async fn main() {
    let state = AppState { /* ... */ };
    let app = App::with_state(8080, create_router(), state);
    app.run().await;
}
```

### Build Script

Every Rejoice project needs `build.rs`:

```rust
fn main() {
    rejoice::codegen::generate_routes();
}
```

---

## Complete Examples

### Simple Blog

**`src/main.rs`**:
```rust
use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
```

**`src/routes/layout.rs`**:
```rust
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    res.html(html! {
        (DOCTYPE)
        html {
            head {
                meta charset="utf-8";
                title { "My Blog" }
            }
            body class="bg-gray-100" {
                nav class="bg-blue-600 text-white p-4" {
                    a href="/" class="font-bold" { "My Blog" }
                    a href="/about" class="ml-4" { "About" }
                }
                main class="container mx-auto p-4" {
                    (children)
                }
            }
        }
    })
}
```

**`src/routes/index.rs`**:
```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    let posts = vec![
        ("Hello World", "My first post"),
        ("Rust is Great", "Why I love Rust"),
    ];
    
    res.html(html! {
        h1 class="text-3xl font-bold mb-4" { "Latest Posts" }
        @for (title, excerpt) in &posts {
            article class="bg-white p-4 rounded shadow mb-4" {
                h2 class="text-xl font-semibold" { (title) }
                p class="text-gray-600" { (excerpt) }
            }
        }
    })
}
```

### User Dashboard with Authentication

**`src/routes/dashboard/layout.rs`**:
```rust
use crate::AppState;
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res {
    // Check authentication
    let session = req.cookies.get("session_id");
    if session.is_none() {
        return res.redirect("/login");
    }
    
    res.html(html! {
        (DOCTYPE)
        html {
            head { title { "Dashboard" } }
            body {
                nav {
                    a href="/dashboard" { "Overview" }
                    a href="/dashboard/settings" { "Settings" }
                    form action="/logout" method="post" {
                        button type="submit" { "Logout" }
                    }
                }
                main { (children) }
            }
        }
    })
}
```

**`src/routes/dashboard/index.rs`**:
```rust
use crate::AppState;
use rejoice::{Req, Res, html, db::query_as};

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let session_id = req.cookies.get("session_id").unwrap();
    
    // Get user from session
    let user: Option<User> = query_as(
        "SELECT u.* FROM users u 
         JOIN sessions s ON s.user_id = u.id 
         WHERE s.token = ?"
    )
    .bind(session_id)
    .fetch_optional(&state.db)
    .await
    .unwrap();
    
    let user = match user {
        Some(u) => u,
        None => return res.delete_cookie("session_id").redirect("/login"),
    };
    
    res.html(html! {
        h1 { "Welcome, " (user.name) "!" }
        // Dashboard content...
    })
}
```

### API Endpoint

**`src/routes/api/users.rs`**:
```rust
use crate::AppState;
use rejoice::{Req, Res, json, db::{query_as, FromRow}};
use serde::Serialize;

#[derive(Serialize, FromRow)]
struct User {
    id: i32,
    name: String,
    email: String,
}

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let users: Vec<User> = query_as("SELECT id, name, email FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    
    res.set_header("Cache-Control", "max-age=60")
       .json(&json!({
           "users": users,
           "count": users.len()
       }))
}
```

### Interactive Component

**`client/TodoList.tsx`**:
```tsx
import { createSignal, For } from "solid-js";

interface TodoListProps {
  initialTodos: string[];
}

export default function TodoList(props: TodoListProps) {
  const [todos, setTodos] = createSignal(props.initialTodos);
  const [newTodo, setNewTodo] = createSignal("");

  const addTodo = () => {
    if (newTodo().trim()) {
      setTodos([...todos(), newTodo().trim()]);
      setNewTodo("");
    }
  };

  const removeTodo = (index: number) => {
    setTodos(todos().filter((_, i) => i !== index));
  };

  return (
    <div class="p-4 bg-white rounded shadow">
      <div class="flex gap-2 mb-4">
        <input
          type="text"
          value={newTodo()}
          onInput={(e) => setNewTodo(e.currentTarget.value)}
          class="flex-1 border rounded px-2 py-1"
          placeholder="Add a todo..."
        />
        <button
          onClick={addTodo}
          class="px-4 py-1 bg-blue-500 text-white rounded"
        >
          Add
        </button>
      </div>
      <ul class="space-y-2">
        <For each={todos()}>
          {(todo, index) => (
            <li class="flex justify-between items-center">
              <span>{todo}</span>
              <button
                onClick={() => removeTodo(index())}
                class="text-red-500 hover:text-red-700"
              >
                Delete
              </button>
            </li>
          )}
        </For>
      </ul>
    </div>
  );
}
```

**`src/routes/todos.rs`**:
```rust
use rejoice::{Req, Res, html, island};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 class="text-2xl font-bold mb-4" { "My Todos" }
        (island!(TodoList, { 
            initialTodos: ["Buy groceries", "Walk the dog", "Learn Rust"]
        }))
    })
}
```

---

## Error Handling

### Returning Error Responses

Use the built-in error helpers for common cases:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res, id: String) -> Res {
    let user = fetch_user(&id).await;
    
    match user {
        Ok(user) => res.html(html! {
            h1 { (user.name) }
        }),
        Err(_) => res.not_found("User not found"),
    }
}
```

Or set status manually for custom responses:

```rust
use rejoice::{Req, Res, html};
use axum::http::StatusCode;

pub async fn get(req: Req, res: Res) -> Res {
    res.set_status(StatusCode::IM_A_TEAPOT)
       .html(html! { h1 { "I'm a teapot" } })
}
```

### API Error Responses

```rust
use rejoice::{Req, Res, json};
use axum::http::StatusCode;

pub async fn get(req: Req, res: Res) -> Res {
    let result = process_request(&req).await;
    
    match result {
        Ok(data) => res.json(&data),
        Err(e) => res.set_status(StatusCode::INTERNAL_SERVER_ERROR)
                     .json(&json!({
                         "error": true,
                         "message": e.to_string()
                     })),
    }
}
```

### Form Validation Example

```rust
use rejoice::{Req, Res, html};
use serde::Deserialize;

#[derive(Deserialize)]
struct SignupForm {
    email: String,
    password: String,
}

pub async fn post(req: Req, res: Res) -> Res {
    let Ok(form) = req.body.as_form::<SignupForm>() else {
        return res.bad_request("Invalid form data");
    };
    
    if form.password.len() < 8 {
        return res.bad_request("Password must be at least 8 characters");
    }
    
    // Create user...
    res.redirect("/welcome")
}
```

### Complete Form Example (GET + POST)

A single route file can handle multiple HTTP methods:

**`src/routes/contact.rs`**:
```rust
use rejoice::{Req, Res, html};
use serde::Deserialize;

#[derive(Deserialize)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

// GET /contact - Display the form
pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Contact Us" }
        form method="post" {
            input type="text" name="name" placeholder="Your name" required;
            input type="email" name="email" placeholder="Your email" required;
            textarea name="message" placeholder="Your message" required {}
            button type="submit" { "Send" }
        }
    })
}

// POST /contact - Handle form submission
pub async fn post(req: Req, res: Res) -> Res {
    let Ok(form) = req.body.as_form::<ContactForm>() else {
        return res.bad_request("Invalid form data");
    };
    
    // Process the form (send email, save to database, etc.)
    println!("Message from {}: {}", form.name, form.message);
    
    res.redirect("/thank-you")
}
```

---

## Deployment

### Building for Production

```bash
rejoice build --release
```

### Running the Binary

The binary must be run from the project root (where `dist/` and `public/` exist):

```bash
./target/release/my-app
```

Or set up a systemd service, Docker container, etc.

### Required Files in Production

```
my-app/
├── target/release/my-app    # The binary
├── dist/                    # Built client assets
│   ├── islands.js
│   └── styles.css
├── public/                  # Static assets
└── .env                     # Environment variables (if using database)
```

### Environment Variables

Set `DATABASE_URL` if using SQLite:
```bash
export DATABASE_URL=sqlite:./my-app.db
./target/release/my-app
```

---

## Troubleshooting

### Common Issues

**Route not found**: Ensure the file is in `src/routes/` and exports an HTTP method function like `pub async fn get(...)` or `pub async fn post(...)`.

**Layout not applying**: Check that `layout.rs` is in the correct directory and exports `pub async fn layout(...)`.

**Island not hydrating**: 
- Ensure the component is exported as `default`
- File must be `.tsx` or `.jsx`
- Component name must match filename exactly

**Database connection errors**: Check `DATABASE_URL` in `.env` and ensure the database file exists.

**Tailwind classes not working**: Run `rejoice dev` or `rejoice build` to rebuild CSS.

**POST data not parsing**: Make sure you're using `req.body.as_form()` for form data or `req.body.as_json()` for JSON payloads.

---

This documentation covers all Rejoice features. For the latest updates, check the source code and AGENTS.md.
