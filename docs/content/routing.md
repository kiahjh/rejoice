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

## HTTP Methods

Route files export functions named after HTTP methods:

```rust
use rejoice::{Req, Res, html};

// Handle GET requests
pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, World!" }
    })
}

// Handle POST requests
pub async fn post(req: Req, res: Res) -> Res {
    // Process form data...
    res.redirect("/success")
}
```

A single route file can export multiple handlers for different HTTP methods.

### Supported Methods

- `get` → GET requests
- `post` → POST requests
- `put` → PUT requests
- `delete` → DELETE requests
- `patch` → PATCH requests

## Index Routes

Files named `index.rs` handle the directory's root path:

- `src/routes/index.rs` → `/`
- `src/routes/blog/index.rs` → `/blog`
- `src/routes/users/index.rs` → `/users`

## Dynamic Routes

Use square brackets for dynamic path segments in file names or directory names.

### Dynamic Files

**`src/routes/users/[id].rs`** handles `/users/:id`:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res, id: String) -> Res {
    res.html(html! {
        h1 { "User " (id) }
    })
}
```

The parameter is passed as the last argument to your handler function.

### Dynamic Directories

You can also use dynamic segments in directory names:

**`src/routes/users/[id]/posts/index.rs`** handles `/users/:id/posts`:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res, id: String) -> Res {
    res.html(html! {
        h1 { "Posts for user " (id) }
    })
}
```

### Multiple Dynamic Segments

Routes can have multiple dynamic parameters:

**`src/routes/users/[user_id]/posts/[post_id]/index.rs`** handles `/users/:user_id/posts/:post_id`:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res, user_id: String, post_id: String) -> Res {
    res.html(html! {
        h1 { "Post " (post_id) " by user " (user_id) }
    })
}
```

Parameters are passed to the handler in the order they appear in the URL path.

### Examples

| File | URL | Parameters |
|------|-----|------------|
| `users/[id].rs` | `/users/123` | `id = "123"` |
| `blog/[slug].rs` | `/blog/hello-world` | `slug = "hello-world"` |
| `users/[id]/posts/index.rs` | `/users/5/posts` | `id = "5"` |
| `users/[uid]/posts/[pid].rs` | `/users/5/posts/42` | `uid = "5"`, `pid = "42"` |

## Route Function Signatures

### Stateless Routes

For apps without shared state (using `routes!()`):

```rust
// Basic route
pub async fn get(req: Req, res: Res) -> Res
pub async fn post(req: Req, res: Res) -> Res

// Dynamic route
pub async fn get(req: Req, res: Res, id: String) -> Res
```

### Stateful Routes

For apps with shared state (using `routes!(AppState)`):

```rust
// Basic route
pub async fn get(state: AppState, req: Req, res: Res) -> Res
pub async fn post(state: AppState, req: Req, res: Res) -> Res

// Dynamic route  
pub async fn get(state: AppState, req: Req, res: Res, id: String) -> Res
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
