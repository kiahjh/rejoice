# Dogfooding Notes

Issues, friction points, and feature ideas discovered while building Rejoice Cloud with the Rejoice framework.

---

## Issues Found

### `.env` file location in workspaces
- **Problem:** When a Rejoice app is part of a Cargo workspace, `dotenvy_macro` looks for `.env` at the workspace root, not the app directory
- **Impact:** Had to copy `cloud/.env` to workspace root for compilation to work
- **Possible fix:** Document this behavior, or find a way to make `rejoice::env!` smarter about workspace setups

### `FromRow` derive requires direct `sqlx` dependency
- **Problem:** Using `#[derive(FromRow)]` from `rejoice::db::FromRow` fails because the macro expands to `sqlx::...` paths
- **Impact:** Users must add `sqlx` directly to their `Cargo.toml` even though Rejoice re-exports `FromRow`
- **Possible fix:** Re-export sqlx itself, or document this requirement, or create a wrapper derive macro

### `island!` macro doesn't support complex expressions
- **Problem:** The `island!` macro's prop values must be simple token trees. Expressions like `!is_finished` or `option_value.clone()` don't work
- **Impact:** Must pre-compute all island props into simple variables before calling the macro
- **Workaround:** Bind complex expressions to variables first, then pass variable names to the macro
- **Possible fix:** Could potentially use a proc-macro approach that evaluates expressions, or document this limitation

### No `http::StatusCode` export
- **Problem:** Rejoice doesn't re-export `axum::http::StatusCode` or similar
- **Impact:** For JSON API routes that need specific status codes, must add axum as a direct dependency or work around it
- **Workaround:** For polling endpoints, just use 200 with status in the JSON body
- **Possible fix:** Add `pub use axum::http::StatusCode;` to `lib.rs` or create error response helpers that return JSON

---

## Feature Requests

### Real-time streaming responses (SSE/WebSocket)
- **Problem:** No built-in support for Server-Sent Events or streaming responses
- **Impact:** Had to implement log streaming via polling instead of true streaming
- **Idea:** Add `res.sse(stream)` or `res.stream()` method for streaming responses

*Add more ideas for framework improvements here as they come up*

---

## Nice-to-Haves

*Lower priority improvements*

---

## Resolved

### Server bound to localhost only (FIXED)
- **Problem:** `App::run()` was binding to `127.0.0.1` which doesn't work in containers/cloud where the proxy needs to connect from outside
- **Impact:** Deployed apps were not reachable - health checks failed because the app only listened on localhost
- **Fix:** Updated `app.rs` to bind to `0.0.0.0` in production (when `REJOICE_DEV` is not set) and `127.0.0.1` in dev mode

### Dynamic route paths using wrong syntax (FIXED)
- **Problem:** Codegen was generating `:param` style paths (Axum 0.7), but Axum 0.8+ uses `{param}`
- **Impact:** Apps with dynamic routes like `[id]` would panic at runtime
- **Fix:** Updated `codegen.rs` to use `{param}` syntax
