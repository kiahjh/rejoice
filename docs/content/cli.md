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

1. Install npm dependencies (if `node_modules/` missing)
2. Generate islands registry
3. Build client assets with Vite
4. Compile Rust binary

**Output:**

| Asset | Location |
|-------|----------|
| Binary | `target/debug/<name>` or `target/release/<name>` |
| JavaScript | `dist/islands.js` |
| CSS | `dist/styles.css` |

## Environment

The CLI expects:

- Rust toolchain (cargo, rustc)
- Node.js and npm
- Project directory with `Cargo.toml`

## Next Steps

- [Installation](/docs/installation) - Getting started
- [Project Structure](/docs/project-structure) - Understanding the files
- [Deployment](/docs/deployment) - Running in production
