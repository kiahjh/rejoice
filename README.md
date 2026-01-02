# Rejoice

A simple and delightful web framework for Rust with file-based routing, layouts, live reload, Tailwind CSS, and SolidJS islands for interactivity.

## Features

- **File-based routing** - Just create `.rs` files in `src/routes/` and they become pages
- **Nested layouts** - Share UI across routes with `layout.rs` files
- **Live reload** - Changes automatically refresh the browser
- **Tailwind CSS v4** - Utility-first CSS that scans your Rust and TSX files
- **SolidJS islands** - Add interactive components without a full SPA
- **Type-safe HTML** - Use Maud for compile-time HTML templating
- **SQLite database** - Optional built-in support with sqlx
- **Zero config** - Just run `rejoice dev` and start building

## Quick Start

```bash
# Install the CLI
cargo install rejoice

# Create a new project
rejoice init my-app
cd my-app

# Start the dev server
rejoice dev
```

To create a project with SQLite database support:

```bash
rejoice init my-app --with-db
```

## File-Based Routing

```
src/routes/
├── layout.rs       -> Wraps all pages
├── index.rs        -> GET /
├── about.rs        -> GET /about
└── users/
    ├── layout.rs   -> Wraps /users/* pages
    ├── index.rs    -> GET /users
    └── [id].rs     -> GET /users/:id
```

Each route file exports an HTTP method handler like `get` or `post`:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello, world!" }
    })
}
```

## Layouts

Layouts wrap pages and nested layouts. Create a `layout.rs` file to share UI:

```rust
use rejoice::{Children, Req, Res, html, DOCTYPE};

pub async fn layout(req: Req, res: Res, children: Children) -> Res {
    res.html(html! {
        (DOCTYPE)
        html {
            head { title { "My App" } }
            body {
                nav { a href="/" { "Home" } }
                main { (children) }
                footer { "Built with Rejoice" }
            }
        }
    })
}
```

Layouts nest automatically. A page at `/users/123` will be wrapped by:
1. `routes/layout.rs` (if exists)
2. `routes/users/layout.rs` (if exists)
3. `routes/users/[id].rs`

## Database Support

Create a project with `--with-db` to get SQLite support out of the box:

```bash
rejoice init my-app --with-db
```

This sets up:
- A SQLite database file
- `.env` with `DATABASE_URL`
- An `AppState` struct with a connection pool
- Routes configured to receive state

Access the database in your routes:

```rust
use crate::AppState;
use rejoice::{Req, Res, db::query_as, html};

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    let users: Vec<(String,)> = query_as("SELECT name FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();

    res.html(html! {
        h1 { "Users" }
        ul {
            @for user in &users {
                li { (user.0) }
            }
        }
    })
}
```

## Custom App State

You can add your own state (database, config, services, etc.) to make it available in all routes:

```rust
use rejoice::{
    App,
    db::{Pool, PoolConfig, Sqlite, create_pool},
};

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
    pub config: AppConfig,
}

rejoice::routes!(AppState);

#[tokio::main]
async fn main() {
    let pool = create_pool(PoolConfig { /* ... */ }).await;
    let state = AppState { db: pool, config: load_config() };

    let app = App::with_state(8080, create_router(), state);
    app.run().await;
}
```

Then access it in routes and layouts:

```rust
pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Use state.db, state.config, etc.
    res.html(html! { /* ... */ })
}
```

## SolidJS Islands

Add interactive components to your pages:

```rust
use rejoice::{Req, Res, html, island};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "My Page" }
        (island!(Counter, { initial: 0 }))
    })
}
```

Create the component in `client/Counter.tsx`:

```tsx
import { createSignal } from "solid-js";

export default function Counter(props: { initial: number }) {
  const [count, setCount] = createSignal(props.initial);
  return (
    <button onClick={() => setCount((c) => c + 1)}>
      Count: {count()}
    </button>
  );
}
```

That's it! The island is automatically registered and hydrated on the client.

## Tailwind CSS

Tailwind CSS v4 is included out of the box. Just use Tailwind classes in your Rust templates or TSX components:

```rust
use rejoice::{Req, Res, html};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 class="text-4xl font-bold text-blue-600" { "Hello!" }
        p class="mt-4 text-gray-700" { "Styled with Tailwind." }
    })
}
```

Tailwind automatically scans your `src/**/*.rs` and `client/**/*.tsx` files for classes.
