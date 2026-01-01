# Installation

Get started with Rejoice in just a few steps.

## Prerequisites

Before you begin, make sure you have the following installed:

- **Rust** (1.85 or later) - [Install Rust](https://rustup.rs)
- **Node.js** (18 or later) - [Install Node.js](https://nodejs.org)

## Install the CLI

Install the Rejoice CLI globally using Cargo:

```bash
cargo install rejoice
```

This gives you access to the `rejoice` command for creating and managing projects.

## Create a New Project

Create a new Rejoice project:

```bash
rejoice init my-app
cd my-app
```

This creates a new directory with a complete project structure including:

- Rust source files with example routes
- Vite configuration for client-side assets
- Tailwind CSS setup
- An example SolidJS island component

### With Database Support

If you need SQLite database support:

```bash
rejoice init my-app --with-db
```

This additionally creates:

- A `.env` file with `DATABASE_URL`
- An empty SQLite database file
- `AppState` struct with a connection pool

## Start the Dev Server

Run the development server:

```bash
rejoice dev
```

Your app is now running at [http://localhost:8080](http://localhost:8080).

The dev server includes:

- **Automatic Rust recompilation** - Changes to `.rs` files trigger a rebuild
- **Vite watch mode** - Client-side assets rebuild automatically  
- **Live reload** - Your browser refreshes when changes are detected

## Build for Production

When you're ready to deploy:

```bash
rejoice build --release
```

This creates an optimized binary at `target/release/my-app` along with the compiled client assets in `dist/`.

## Next Steps

- [Project Structure](/docs/project-structure) - Understand how your project is organized
- [Routing](/docs/routing) - Learn about file-based routing
- [Templates](/docs/templates) - Write HTML with Maud
