# Rejoice - Agent Guide

This document provides technical context for AI coding agents working on the Rejoice framework.

## Project Structure

```
src/
├── bin/
│   ├── main.rs              # CLI entry point (uses clap)
│   └── commands/
│       ├── mod.rs           # Command exports
│       ├── init.rs          # `rejoice init` - project scaffolding
│       ├── dev.rs           # `rejoice dev` - dev server with HMR
│       ├── islands.rs       # Generates client/islands.tsx registry
│       └── style.rs         # Terminal output helpers
├── assets/
│   └── live_reload.js       # Client-side HMR script (injected into HTML)
├── app.rs                   # App struct, middleware, server setup
├── codegen.rs               # Build-time route generation
├── db.rs                    # SQLite pool config and exports
├── env.rs                   # Re-exports dotenvy_macro::dotenv as env!
├── island.rs                # Island macro for SolidJS components
├── request.rs               # Req type for incoming request data
├── response.rs              # Res type for building responses
└── lib.rs                   # Public API exports
```

## CLI Commands

The CLI uses **clap** with derive macros. Defined in `src/bin/main.rs`.

### `rejoice init [name] [--with-db]`

Creates a new project. Implementation in `src/bin/commands/init.rs`.

**Without `--with-db`:**
- Basic project with `App::new()` and `routes!()`
- Routes receive `req: Req, res: Res`
- No database files

**With `--with-db`:**
- Creates `.env` with `DATABASE_URL` and empty `.db` file
- Generates `AppState` struct with db pool in `main.rs`
- Uses `App::with_state()` and `routes!(AppState)`
- Routes receive `state: AppState, req: Req, res: Res`

**IMPORTANT:** When changing the framework's public API, imports, or patterns, update the generated templates in `init.rs` to match.

### `rejoice dev`

Starts the dev server with:
- Cargo watch for Rust recompilation
- Vite watch for client assets
- WebSocket-based live reload

### `rejoice build [--release]`

Builds the project for deployment. Implementation in `src/bin/commands/build.rs`.

**Steps performed:**
1. Install npm dependencies (if `node_modules/` missing and `client/` exists)
2. Generate islands registry (if `client/` exists)
3. Build client assets with Vite (if `client/` exists)
4. Build Rust binary with Cargo

**Flags:**
- `--release` - Build with optimizations, prints deployment instructions

**Output locations:**
- Binary: `target/debug/<name>` or `target/release/<name>`
- Client assets: `dist/islands.js`, `dist/styles.css`

## Code Generation

The `codegen.rs` module runs at **build time** via the user's `build.rs`:

```rust
fn main() {
    rejoice::codegen::generate_routes();
}
```

### What it generates

1. **`src/routes.rs`** - Module declarations for rust-analyzer support
2. **`$OUT_DIR/routes_generated.rs`** - Actual router code, included via `routes!()` macro

### Route discovery

Scans `src/routes/` recursively:
- `index.rs` → `/` or `/parent`
- `about.rs` → `/about`
- `[id].rs` → `/:id` (dynamic segment)
- `layout.rs` → Wrapper for sibling/child routes

### Generated wrapper functions

For routes with layouts, generates wrapper functions that:
1. Extract state via Axum's `State` extractor (internally)
2. Extract `Req` and `Res` from the request
3. Call the page function with `(state, req, res)` or `(req, res)`
4. If the response is HTML, wrap with layouts (innermost to outermost)
5. If the response is not HTML (redirect, JSON, etc.), return it directly without layout wrapping

The `__RejoiceState` type alias is defined by the `routes!()` macro:
- `routes!()` → `type __RejoiceState = ();`
- `routes!(AppState)` → `type __RejoiceState = AppState;`

### Router generation

```rust
pub fn create_router() -> axum::Router<__RejoiceState> {
    axum::Router::new()
        .route("/", axum::routing::get(wrapper_index))
        // ... more routes
}
```

## Request and Response Types

### `Req` - Incoming Request

The `Req` type provides read-only access to request data:

```rust
pub struct Req {
    pub headers: HeaderMap,   // HTTP headers
    pub cookies: Cookies,     // Parsed cookies
    pub method: Method,       // GET, POST, etc.
    pub uri: Uri,             // Request URI
}

// Reading request data
let auth = req.headers.get("Authorization");
let session = req.cookies.get("session_id");
```

### `Res` - Response Builder

The `Res` type uses interior mutability for building responses.

**Mutators** (return `&Res` for chaining):
- `set_cookie(name, value)` - Set a cookie
- `set_cookie_with_options(...)` - Set cookie with path, max_age, etc.
- `delete_cookie(name)` - Delete a cookie
- `set_header(name, value)` - Set a response header
- `set_status(StatusCode)` - Override status code

**Finalizers** (consume `Res`, return `Res`):
- `html(Markup)` - HTML response (200, text/html)
- `json(&impl Serialize)` - JSON response (200, application/json)
- `redirect(url)` - 302 redirect
- `redirect_permanent(url)` - 301 redirect
- `raw(impl Into<Vec<u8>>)` - Raw bytes

**Example usage:**

```rust
pub async fn page(state: AppState, req: Req, res: Res) -> Res {
    // Read cookies
    let session = req.cookies.get("session");
    
    if session.is_none() {
        // Redirect (bypasses layout wrapping)
        return res.redirect("/login");
    }
    
    // Set cookies and return HTML
    res.set_cookie("last_visit", "2025-01-01")
       .set_header("X-Custom", "value")
       .html(html! {
           h1 { "Dashboard" }
       })
}

// API endpoint returning JSON
pub async fn users(state: AppState, req: Req, res: Res) -> Res {
    let users = get_users(&state.db).await;
    res.json(&users)
}
```

## App and State

### Stateless apps

```rust
let app = App::new(8080, create_router());
```

### Stateful apps

```rust
let app = App::with_state(8080, create_router(), state);
```

`App::with_state()` is generic over any `S: Clone + Send + Sync + 'static`. The state is attached to the router via Axum's `.with_state()` before serving.

### Route signatures

Routes and layouts receive state as a plain value (not wrapped in `State`):

```rust
// Stateless
pub async fn page(req: Req, res: Res) -> Res { ... }
pub async fn layout(req: Req, res: Res, children: Children) -> Res { ... }

// Stateful  
pub async fn page(state: AppState, req: Req, res: Res) -> Res { ... }
pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res { ... }
```

Note: The codegen handles Axum's `State` extraction internally; user code receives the unwrapped state value.

## Database Support

**Feature-gated:** The database module requires the `sqlite` feature flag.

```toml
# In user's Cargo.toml
rejoice = { version = "...", features = ["sqlite"] }
```

Exports in `src/db.rs` (only available with `sqlite` feature):
- `Pool`, `Sqlite` - sqlx types
- `query`, `query_as` - sqlx query functions/macros
- `PoolConfig`, `create_pool` - Pool creation helpers

Users access via `rejoice::db::*`.

When `rejoice init --with-db` is used, the generated `Cargo.toml` automatically includes the `sqlite` feature.

## Islands (SolidJS Components)

### How islands work

1. User creates TSX component in `client/ComponentName.tsx`
2. User uses `island!(ComponentName, { props })` macro in Rust
3. The macro generates a `<div data-island="ComponentName" data-props='{"props": ...}'>`
4. Vite builds `client/islands.tsx` (auto-generated) which registers all components
5. Client-side JS finds `[data-island]` elements and hydrates them with SolidJS

### The island macro

Defined in `src/island.rs`. Generates:
- Wrapper div with `data-island` attribute (component name)
- `data-props` attribute with JSON-serialized props (HTML-escaped)

### Islands registry generation

`src/bin/commands/islands.rs` contains `generate_islands_registry()` which:
1. Scans `client/` for `.tsx` and `.jsx` files (excluding `islands.tsx` itself)
2. Generates `client/islands.tsx` with imports and a registry object
3. Includes hydration code that queries `[data-island]` elements and renders SolidJS components
4. Exposes `window.__hydrateIslands()` for re-hydration after HMR

This runs automatically during `rejoice dev` on startup and when client files change.

## Hot Module Replacement

### How HMR works

1. `dev.rs` starts a WebSocket server on port 3001 at `/__reload`
2. `assets/live_reload.js` is injected into HTML responses (via middleware in `app.rs`)
3. File watchers detect changes to Rust or client files
4. On change: rebuild triggered, WebSocket sends reload message
5. Client receives message and reloads

### Reload types

- `"full"` - Client JS changed; triggers full `location.reload()`
- `"reload"` - Rust changed; fetches new HTML, swaps `document.body`, re-hydrates islands via `window.__hydrateIslands()`

### Script injection

`ScriptInjectionMiddleware` in `app.rs`:
- Checks if response is HTML
- Injects `<script>` before `</body>` for islands and live reload
- Injects `<link>` in `</head>` for styles

## Static Assets (public/)

The `public/` directory serves static files at the root URL path:
- `public/logo.png` → `/logo.png`
- `public/images/hero.jpg` → `/images/hero.jpg`
- `public/favicon.ico` → `/favicon.ico`

Implemented in `app.rs` using `fallback_service(ServeDir::new("public"))`, so defined routes take precedence over static files.

The `public/` directory is watched during `rejoice dev` and triggers a reload when files change.

## Tailwind CSS

Configured in the generated `vite.config.ts` with `@tailwindcss/vite` plugin.

`client/styles.css` contains:
```css
@import "tailwindcss";
@source "../src/**/*.rs";
@source "./**/*.tsx";
```

This tells Tailwind to scan Rust and TSX files for class names.

## Public Exports

From `src/lib.rs`:

**Root level:**
- `App` - Server struct
- `Req` - Incoming request data (headers, cookies, method, uri)
- `Res` - Response builder with `set_*` mutators and finalizers
- `Children` - Type alias for layout children (`Markup`)
- `Path` - Axum path extractor for dynamic routes
- `html!`, `Markup`, `DOCTYPE`, `PreEscaped` - Maud re-exports (flattened)
- `json` - serde_json::json macro
- `island!`, `island_fn` - Island support
- `routes!` - Include generated routes

**Prelude module:**
```rust
use rejoice::prelude::*;
// Brings in: App, Req, Res, Children, Path, html, Markup, DOCTYPE, PreEscaped, json, island
```

**Feature-gated:**
- `db::*` - SQLite support (requires `sqlite` feature)

**Internal (doc-hidden):**
- `State`, `Router`, `routing` - Used by generated code

## Dependencies

**IMPORTANT:** All dependencies in `Cargo.toml` must use exact versions (e.g., `"1.0.148"` not `"1"`). When adding or updating dependencies, always pin to a specific patch version.

## Maintenance Checklist

When modifying the framework:

1. **Changing public API or imports** → Update `init.rs` templates
2. **Changing route/layout signatures** → Update `codegen.rs` wrapper generation AND `init.rs`
3. **Adding new exports** → Update `lib.rs` and this document
4. **Changing CLI commands** → Update clap definitions in `main.rs`
5. **Changing generated project structure** → Update `init.rs` step count and file generation
6. **Any significant changes** → Update this `AGENTS.md` file
7. **ANY change to the framework** → Update `LLM_DOCS.md` to reflect the change. This file is the comprehensive user-facing documentation for AI agents building apps with Rejoice. It MUST stay perfectly in sync with the actual framework behavior. When in doubt, update it.
8. **ANY change to the framework** → Update `README.md` if the change affects user-facing features, API usage examples, or getting started instructions. The README is the first thing users see, so it must accurately reflect how the framework works.
