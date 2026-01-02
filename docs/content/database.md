# Database

Rejoice provides optional SQLite support via [sqlx](https://github.com/launchbadge/sqlx).

## Quick Setup

```bash
rejoice init my-app --with-db
```

This creates an `AppState` with a connection pool, `.env` with `DATABASE_URL`, and an empty `.db` file.

## Manual Setup

Enable the feature in `Cargo.toml`:

```toml
[dependencies]
rejoice = { version = "0.10.0", features = ["sqlite"] }
```

Create `.env`:

```text
DATABASE_URL=sqlite:./my-app.db
```

Configure your app:

```rust
use std::time::Duration;
use rejoice::{App, db::{Pool, PoolConfig, Sqlite, create_pool}};

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

## Queries

```rust
use rejoice::{Req, Res, html, db::{query, query_as, query_scalar, FromRow}};

#[derive(FromRow)]
struct User { id: i32, name: String }

pub async fn get(state: AppState, req: Req, res: Res) -> Res {
    // Typed query
    let users: Vec<User> = query_as("SELECT id, name FROM users")
        .fetch_all(&state.db)
        .await
        .unwrap();
    
    // With parameters
    let user: Option<User> = query_as("SELECT * FROM users WHERE id = ?")
        .bind(123)
        .fetch_optional(&state.db)
        .await
        .unwrap();
    
    // Scalar query (for COUNT, MAX, single values)
    let count: i32 = query_scalar("SELECT COUNT(*) FROM users")
        .fetch_one(&state.db)
        .await
        .unwrap();
    
    // Insert/update/delete
    query("INSERT INTO users (name) VALUES (?)")
        .bind("Alice")
        .execute(&state.db)
        .await
        .unwrap();
    
    res.html(html! { /* ... */ })
}
```

For complete sqlx documentation, see [docs.rs/sqlx](https://docs.rs/sqlx).
