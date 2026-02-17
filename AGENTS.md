# Rejoice - Agent Guide

This document provides strict instructions and technical context for AI coding agents working on the Rejoice framework.

---

## Core Principles (MANDATORY)

These principles are non-negotiable. Every agent must follow them at all times.

### 1. Excellence Over Convenience

Always do things the **best and most correct way**, not just the easiest or quickest way. This means:
- Choose robust, maintainable solutions over quick hacks
- Follow established patterns and idioms for Rust and the web ecosystem
- Consider edge cases, error handling, and future maintainability
- If you're unsure whether an approach is correct, research it before implementing

### 2. Comprehensive Testing

Everything that can be tested **must** have thorough tests:
- Write tests for all new functionality
- Cover edge cases, error conditions, and boundary values
- Tests must pass before considering work complete
- If you find untested code, add tests for it
- Run `cargo test` frequently to ensure nothing is broken

### 3. Codebase Excellence

The codebase must remain in **impeccable condition**:
- Be proactive: if you encounter poorly written, outdated, or incorrect code, fix it—even if it's not what you're currently working on
- Never assume existing code is correct just because it exists; verify and improve
- Maintain consistent style, naming conventions, and patterns throughout
- Remove dead code, clean up TODOs, and address technical debt as you find it
- Leave every file better than you found it

### 4. Dogfooding Awareness

When working on **Rejoice Cloud** (the `cloud/` directory), you are using Rejoice to build a real application. This is a critical dogfooding opportunity:
- Document any framework friction, bugs, or missing features in `DOGFOODING_NOTES.md`
- Even small annoyances matter—write them down
- Suggest or implement framework improvements based on real usage

---

## Rejoice Cloud UI Guidelines (MANDATORY)

When working on the Rejoice Cloud UI (`cloud/` directory), follow these rules strictly:

### 1. Use the Component System

All UI must be built using the component system in `cloud/src/components/`. **Never** write raw HTML with inline styles.

```rust
// GOOD - Use components
use crate::components::{self as ui, ButtonVariant, ButtonSize};

ui::button_link("/projects", "View Projects", ButtonVariant::Primary, ButtonSize::Medium)
ui::card(html! { ... })
ui::page_header("Title", Some("Subtitle"), Some(actions))

// BAD - Raw HTML with inline styles
html! { a href="/projects" style="background: blue; padding: 10px;" { "View Projects" } }
```

### 2. Use Tailwind CSS Exclusively

- **Never** use inline `style` attributes
- **Never** write custom CSS unless absolutely necessary (only for things Tailwind cannot do)
- All styling must use Tailwind utility classes
- The CSS file should only contain: Tailwind imports, font imports, and minimal base styles

### 3. Component Structure

Components live in `cloud/src/components/`:
- `button.rs` - Buttons, links styled as buttons, nav links
- `card.rs` - Cards, badges, project cards
- `form.rs` - Inputs, labels, form groups
- `icon.rs` - SVG icons as functions
- `layout.rs` - Page structure, grids, empty states
- `typography.rs` - Text styles

### 4. When to Create New Components

Create a new component when:
- A UI pattern is used more than once
- The element has variants (sizes, colors, states)
- The markup is complex enough to warrant abstraction

### 5. Component Design Patterns

```rust
// Use enums for variants
pub enum ButtonVariant { Primary, Secondary, Ghost, Danger }
pub enum ButtonSize { Small, Medium, Large }

// Components return Markup
pub fn button(label: &str, variant: ButtonVariant, size: ButtonSize) -> Markup

// Components that accept children
pub fn card(children: Markup) -> Markup

// Components compose other components
pub fn project_card(...) -> Markup {
    html! {
        (card(html! {
            // Uses card internally
        }))
    }
}
```

---

## Project Structure

```
rejoice/                     # Main framework crate
├── src/
│   ├── bin/
│   │   ├── main.rs          # CLI entry point (clap)
│   │   └── commands/
│   │       ├── mod.rs       # Command exports
│   │       ├── init.rs      # `rejoice init` - project scaffolding
│   │       ├── dev.rs       # `rejoice dev` - dev server with HMR
│   │       ├── build.rs     # `rejoice build` - production builds
│   │       ├── migrate.rs   # `rejoice migrate` - database migrations
│   │       ├── boilerplate.rs
│   │       ├── islands.rs   # Generates client/islands.tsx registry
│   │       └── style.rs     # Terminal output helpers
│   ├── assets/
│   │   └── live_reload.js   # Client-side HMR script
│   ├── app.rs               # App struct, middleware, server setup
│   ├── codegen.rs           # Build-time route generation
│   ├── db.rs                # SQLite pool config (feature-gated)
│   ├── env.rs               # Re-exports dotenvy_macro::dotenv as env!
│   ├── island.rs            # Island macro for SolidJS components
│   ├── request.rs           # Req type
│   ├── response.rs          # Res type
│   └── lib.rs               # Public API exports
cloud/                       # Rejoice Cloud - deployment platform (dogfooding!)
docs/                        # Documentation website
```

---

## CLI Commands

The CLI uses **clap** with derive macros. Defined in `rejoice/src/bin/main.rs`.

### `rejoice init [name] [--with-db]`

Creates a new project. Implementation: `commands/init.rs`.

- **Without `--with-db`:** Basic project with `App::new()` and `routes!()`
- **With `--with-db`:** Adds `.env`, database file, `AppState` with pool, uses `App::with_state()` and `routes!(AppState)`

### `rejoice dev`

Development server with:
- Cargo watch for Rust recompilation
- Vite watch for client assets (via Bun)
- WebSocket-based live reload
- Auto-generates boilerplate for new route/layout files

### `rejoice build [--release]`

Production build:
1. Install JS dependencies with Bun (if needed)
2. Generate islands registry (if `client/` exists)
3. Build client assets with Vite
4. Build Rust binary with Cargo

### `rejoice migrate <action>`

Database migrations via sqlx-cli:
- `add <name>` - Create new migration
- `up` - Apply pending migrations
- `revert` - Revert last migration
- `status` - Show migration status

---

## Code Generation

`codegen.rs` runs at build time via the user's `build.rs`:

```rust
fn main() {
    rejoice::codegen::generate_routes();
}
```

### Route Discovery

Scans `src/routes/` recursively:
- `index.rs` → `/` or `/parent`
- `about.rs` → `/about`
- `[id].rs` → `/:id` (dynamic segment)
- `layout.rs` → Wrapper for sibling/child routes

### HTTP Methods

Route files export functions named after HTTP methods: `get`, `post`, `put`, `delete`, `patch`.

### Generated Code

- `src/routes.rs` - Module declarations for rust-analyzer
- `$OUT_DIR/routes_generated.rs` - Router code, included via `routes!()` macro

---

## Request and Response Types

### `Req` - Incoming Request

```rust
pub struct Req {
    pub headers: HeaderMap,
    pub cookies: Cookies,
    pub method: Method,
    pub uri: Uri,
    pub body: Body,
}

// Usage
let auth = req.headers.get("Authorization");
let session = req.cookies.get("session_id");
let form = req.body.as_form::<MyForm>()?;
let json = req.body.as_json::<MyData>()?;
```

### `Res` - Response Builder

Uses interior mutability for chaining.

**Mutators** (return `&Res`):
- `set_cookie(name, value)`
- `set_cookie_with_options(...)`
- `delete_cookie(name)`
- `set_header(name, value)`
- `set_status(StatusCode)`

**Finalizers** (return owned `Res`):
- `html(Markup)` - HTML response
- `json(&impl Serialize)` - JSON response
- `redirect(url)` - 302 redirect
- `redirect_permanent(url)` - 301 redirect
- `raw(impl Into<Vec<u8>>)` - Raw bytes

**Error helpers:**
- `bad_request(msg)` - 400
- `unauthorized(msg)` - 401
- `forbidden(msg)` - 403
- `not_found(msg)` - 404
- `internal_error(msg)` - 500

---

## Route Signatures

```rust
// Stateless
pub async fn get(req: Req, res: Res) -> Res { ... }
pub async fn layout(req: Req, res: Res, children: Children) -> Res { ... }

// Stateful
pub async fn get(state: AppState, req: Req, res: Res) -> Res { ... }
pub async fn layout(state: AppState, req: Req, res: Res, children: Children) -> Res { ... }
```

---

## Database Support

Feature-gated with `sqlite`:

```toml
rejoice = { version = "...", features = ["sqlite"] }
```

Exports: `Pool`, `Sqlite`, `query`, `query_as`, `query_scalar`, `FromRow`, `PoolConfig`, `create_pool`

---

## Islands (SolidJS Components)

1. Create TSX component in `client/ComponentName.tsx`
2. Use `island!(ComponentName, { props })` macro in Rust
3. Framework generates `<div data-island="ComponentName" data-props='...'>`
4. Client-side JS hydrates with SolidJS

---

## Dependencies

**IMPORTANT:** All dependencies must use exact versions (e.g., `"1.0.148"` not `"1"`). Always pin to specific patch versions.

---

## Maintenance Checklist

When modifying the framework:

1. **Changing public API or imports** → Update `init.rs` templates
2. **Changing route/layout signatures** → Update `codegen.rs` AND `init.rs`
3. **Adding new exports** → Update `lib.rs` and this document
4. **Changing CLI commands** → Update clap definitions in `main.rs`
5. **Changing generated project structure** → Update `init.rs` step count and file generation
6. **Any significant changes** → Update this `AGENTS.md` file
7. **ANY framework change** → Update `llms.txt` and `llms-full.txt`
8. **ANY framework change** → Update `/docs` website
9. **ANY framework change** → Update `README.md` if it affects user-facing features
10. **When dogfooding** → Document friction in `DOGFOODING_NOTES.md`

---

## Key Files Reference

| File | Purpose |
|------|---------|
| `AGENTS.md` | This file - agent instructions |
| `DOGFOODING_NOTES.md` | Framework issues found while building Cloud |
| `REJOICE_CLOUD_PLAN.md` | Technical plan for Rejoice Cloud |
| `llms.txt` / `llms-full.txt` | User-facing AI documentation |
| `README.md` | Public-facing project intro |
