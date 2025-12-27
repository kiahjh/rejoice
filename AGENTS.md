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
├── html.rs                  # Re-exports maud (html!, Markup, etc.)
├── island.rs                # Island macro for SolidJS components
└── lib.rs                   # Public API exports
```

## CLI Commands

The CLI uses **clap** with derive macros. Defined in `src/bin/main.rs`.

### `rejoice init [name] [--with-db]`

Creates a new project. Implementation in `src/bin/commands/init.rs`.

**Without `--with-db`:**
- Basic project with `App::new()` and `routes!()`
- Routes use `State<()>`
- No database files

**With `--with-db`:**
- Creates `.env` with `DATABASE_URL` and empty `.db` file
- Generates `AppState` struct with db pool in `main.rs`
- Uses `App::with_state()` and `routes!(AppState)`
- Routes use `State<AppState>`

**IMPORTANT:** When changing the framework's public API, imports, or patterns, update the generated templates in `init.rs` to match.

### `rejoice dev`

Starts the dev server with:
- Cargo watch for Rust recompilation
- Vite watch for client assets
- WebSocket-based live reload

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
1. Accept `State<__RejoiceState>` (type alias set by `routes!()` macro)
2. Call the page function with state
3. Wrap result in each layout (innermost to outermost)
4. Pass state to each layout

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

All routes and layouts must accept state as the first parameter:

```rust
// Stateless
pub async fn page(State(_): State<()>) -> Markup { ... }
pub async fn layout(State(_): State<()>, children: Children) -> Markup { ... }

// Stateful  
pub async fn page(State(state): State<AppState>) -> Markup { ... }
pub async fn layout(State(state): State<AppState>, children: Children) -> Markup { ... }
```

## Database Support

Exports in `src/db.rs`:
- `Pool`, `Sqlite` - sqlx types
- `query`, `query_as` - sqlx query functions/macros
- `PoolConfig`, `create_pool` - Pool creation helpers

Users access via `rejoice::db::*`.

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
- `App` - Server struct
- `State`, `Path` - Axum extractors
- `Children` - Type alias for layout children (`Markup`)
- `island_fn`, `island!` - Island support
- `json` - serde_json re-export
- `routes!` - Include generated routes
- `html::*` - Maud re-exports (`html!`, `Markup`, `DOCTYPE`, `PreEscaped`)
- `db::*` - SQLite support
- `env::*` - Environment variable macro
- `codegen::*` - Build-time route generation

## Maintenance Checklist

When modifying the framework:

1. **Changing public API or imports** → Update `init.rs` templates
2. **Changing route/layout signatures** → Update `codegen.rs` wrapper generation AND `init.rs`
3. **Adding new exports** → Update `lib.rs` and this document
4. **Changing CLI commands** → Update clap definitions in `main.rs`
5. **Changing generated project structure** → Update `init.rs` step count and file generation
6. **Any significant changes** → Update this `AGENTS.md` file
