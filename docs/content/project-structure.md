# Project Structure

A Rejoice project follows a predictable structure that makes it easy to find and organize your code.

## Directory Layout

```text
my-app/
├── build.rs                 # Build script for route generation
├── Cargo.toml               # Rust dependencies
├── package.json             # Node.js dependencies
├── vite.config.ts           # Vite configuration
├── tsconfig.json            # TypeScript configuration
├── .env                     # Environment variables (optional)
├── .gitignore
│
├── client/                  # Client-side code
│   ├── styles.css           # Tailwind CSS entry point
│   ├── Counter.tsx          # Example SolidJS component
│   └── islands.tsx          # Auto-generated (do not edit)
│
├── public/                  # Static assets served at root URL
│   └── (images, fonts, etc.)
│
├── dist/                    # Built client assets (auto-generated)
│   ├── islands.js
│   └── styles.css
│
└── src/
    ├── main.rs              # Application entry point
    ├── routes.rs            # Auto-generated route modules
    └── routes/              # File-based routes
        ├── layout.rs        # Root layout
        ├── index.rs         # GET /
        └── ...
```

## Key Files

### `src/main.rs`

The entry point for your application. It creates the app and starts the server:

```rust
use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
```

### `build.rs`

The build script that generates routes at compile time:

```rust
fn main() {
    rejoice::codegen::generate_routes();
}
```

This scans your `src/routes/` directory and generates the router code.

### `src/routes/`

This directory contains your route files. Each file becomes a route:

| File | URL |
|------|-----|
| `routes/index.rs` | `/` |
| `routes/about.rs` | `/about` |
| `routes/blog/index.rs` | `/blog` |
| `routes/blog/[slug].rs` | `/blog/:slug` |
| `routes/layout.rs` | (wraps all routes) |

### `client/`

Client-side code lives here:

- **`styles.css`** - Your Tailwind CSS entry point
- **`*.tsx`** - SolidJS island components
- **`islands.tsx`** - Auto-generated registry (don't edit this)

### `public/`

Static assets are served directly from this directory:

- `public/logo.png` → `/logo.png`
- `public/images/hero.jpg` → `/images/hero.jpg`

### `dist/`

Built client assets are output here. This is auto-generated during build and should be in your `.gitignore`.

## Auto-generated Files

Some files are generated automatically and should not be edited:

- **`src/routes.rs`** - Route module declarations
- **`client/islands.tsx`** - Island component registry
- **`dist/`** - Built client assets

These are regenerated when you run `rejoice dev` or `rejoice build`.

## Next Steps

- [Routing](/docs/routing) - Learn how file-based routing works
- [Layouts](/docs/layouts) - Understand nested layouts
