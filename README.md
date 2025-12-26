# Rejoice

A simple and delightful web framework for Rust with file-based routing, live reload, and SolidJS islands for interactivity.

## Features

- **File-based routing** - Just create `.rs` files in `src/routes/` and they become pages
- **Live reload** - Changes automatically refresh the browser
- **SolidJS islands** - Add interactive components without a full SPA
- **Type-safe HTML** - Use Maud for compile-time HTML templating
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

## File-Based Routing

```
src/routes/
├── index.rs        → GET /
├── about.rs        → GET /about
└── users/
    ├── index.rs    → GET /users
    └── [id].rs     → GET /users/:id
```

Each route file exports a `handler` function:

```rust
use rejoice::{html, Markup};

pub async fn handler() -> Markup {
    html! {
        h1 { "Hello, world!" }
    }
}
```

## SolidJS Islands

Add interactive components to your pages:

```rust
use rejoice::{html, island, Markup, DOCTYPE};

pub async fn handler() -> Markup {
    html! {
        (DOCTYPE)
        html {
            body {
                h1 { "My Page" }
                (island!(Counter, { "initial": 0 }))
            }
        }
    }
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
