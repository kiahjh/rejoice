# Routing

Rejoice uses file-based routing. Every `.rs` file in `src/routes/` becomes a route automatically.

## File-to-URL Mapping

| File Path | URL |
|-----------|-----|
| `src/routes/index.rs` | `/` |
| `src/routes/about.rs` | `/about` |
| `src/routes/contact.rs` | `/contact` |
| `src/routes/blog/index.rs` | `/blog` |
| `src/routes/blog/post.rs` | `/blog/post` |
| `src/routes/users/[id].rs` | `/users/:id` |

### Naming Convention

File names with underscores are converted to hyphens in URLs:

| File | URL |
|------|-----|
| `src/routes/about_us.rs` | `/about-us` |
| `src/routes/contact_form.rs` | `/contact-form` |

## Basic Route

Every route file must export an async `page` function:

```rust
use rejoice::{Req, Res, html};

pub async fn page(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}
```

## Index Routes

Files named `index.rs` handle the directory's root path:

- `src/routes/index.rs` → `/`
- `src/routes/blog/index.rs` → `/blog`
- `src/routes/users/index.rs` → `/users`

## Dynamic Routes

Use square brackets for dynamic path segments:

**`src/routes/users/[id].rs`** handles `/users/:id`:

```rust
use rejoice::{Req, Res, html};

pub async fn page(req: Req, res: Res, id: String) -> Res {
    res.html(html! {
        h1 { "User " (id) }
    })
}
```

The parameter is passed as the last argument to your `page` function.

### Examples

| File | URL | Parameter |
|------|-----|-----------|
| `[id].rs` | `/users/123` | `id = "123"` |
| `[slug].rs` | `/blog/hello-world` | `slug = "hello-world"` |
| `[category].rs` | `/products/electronics` | `category = "electronics"` |

## Route Function Signatures

### Stateless Routes

For apps without shared state (using `routes!()`):

```rust
// Basic route
pub async fn page(req: Req, res: Res) -> Res

// Dynamic route
pub async fn page(req: Req, res: Res, id: String) -> Res
```

### Stateful Routes

For apps with shared state (using `routes!(AppState)`):

```rust
// Basic route
pub async fn page(state: AppState, req: Req, res: Res) -> Res

// Dynamic route  
pub async fn page(state: AppState, req: Req, res: Res, id: String) -> Res
```

## Nested Directories

Create nested routes by adding subdirectories:

```text
src/routes/
├── index.rs          → /
├── about.rs          → /about
└── blog/
    ├── index.rs      → /blog
    ├── [slug].rs     → /blog/:slug
    └── archive/
        └── index.rs  → /blog/archive
```

## Next Steps

- [Layouts](/docs/layouts) - Wrap routes with shared UI
- [Request Object](/docs/request) - Access request data
- [Response Object](/docs/response) - Build responses
