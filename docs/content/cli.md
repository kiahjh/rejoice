# CLI Commands

The Rejoice CLI helps you create, develop, and build projects.

## Installation

```bash
cargo install rejoice
```

## Commands

### `rejoice init`

Create a new project.

```bash
# Create in a new directory
rejoice init my-app

# Create with database support
rejoice init my-app --with-db

# Initialize in current directory
cd my-empty-dir
rejoice init
```

**Options:**

| Option | Description |
|--------|-------------|
| `[name]` | Project name (optional, uses directory name if omitted) |
| `--with-db` | Include SQLite database support |

**What it creates:**

Without `--with-db`:
- Basic Cargo.toml with Rejoice dependency
- `src/main.rs` with `App::new()` setup
- `src/routes/layout.rs` and `src/routes/index.rs`
- `build.rs` for route generation
- `client/` with Vite, Tailwind, and example SolidJS component
- `package.json`, `vite.config.ts`, `tsconfig.json`

With `--with-db`:
- Everything above, plus:
- `.env` with `DATABASE_URL`
- Empty `.db` SQLite file
- `AppState` struct with db pool
- `App::with_state()` setup

### `rejoice dev`

Start the development server.

```bash
rejoice dev
```

**Features:**

- Runs your app at `http://localhost:8080`
- Watches Rust files and recompiles on changes
- Watches client files and rebuilds with Vite
- Live reload via WebSocket (`ws://localhost:3001/__reload`)
- Hot module replacement for islands
- **Auto-generates boilerplate** for new route and layout files

**Auto-generated boilerplate:**

When you create a new `.rs` file in `src/routes/` while the dev server is running, it automatically populates the file with appropriate starter code:

- Route files get a `get` handler with the correct signature
- Layout files get a layout function with `DOCTYPE` and children
- Dynamic routes like `[id].rs` include the path parameter
- If your app uses `AppState`, the boilerplate includes it

**When to restart manually:**

- After adding new dependencies to `Cargo.toml`
- After modifying `build.rs`

### `rejoice build`

Build for production.

```bash
# Development build
rejoice build

# Production build (optimized)
rejoice build --release
```

**Options:**

| Option | Description |
|--------|-------------|
| `--release` | Build with optimizations |

**Build steps:**

1. Install dependencies with Bun (if `node_modules/` missing)
2. Generate islands registry
3. Build client assets with Vite
4. Compile Rust binary

**Output:**

| Asset | Location |
|-------|----------|
| Binary | `target/debug/<name>` or `target/release/<name>` |
| JavaScript | `dist/islands.js` |
| CSS | `dist/styles.css` |

### `rejoice migrate`

Manage database migrations (requires `--with-db` project setup).

```bash
# Create a new migration
rejoice migrate add create_users_table

# Apply pending migrations
rejoice migrate up

# Revert the last migration
rejoice migrate revert

# Check migration status
rejoice migrate status
```

**Subcommands:**

| Command | Description |
|---------|-------------|
| `add <name>` | Create a new reversible migration (up.sql + down.sql) |
| `up` | Apply all pending migrations |
| `revert` | Revert the last applied migration |
| `status` | Show which migrations have been applied |

Migrations are stored in the `migrations/` directory. If `sqlx-cli` is not installed, the command will offer to install it automatically.

## Environment

The CLI expects:

- Rust toolchain (cargo, rustc)
- Bun
- Project directory with `Cargo.toml`

## Next Steps

- [Installation](/docs/installation) - Getting started
- [Project Structure](/docs/project-structure) - Understanding the files
- [Deployment](/docs/deployment) - Running in production
