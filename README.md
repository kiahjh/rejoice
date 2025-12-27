# Rejoice

A simple and delightful web framework for Rust with file-based routing, layouts, live reload, Tailwind CSS, and SolidJS islands for interactivity.

## Features

- **File-based routing** - Just create `.rs` files in `src/routes/` and they become pages
- **Nested layouts** - Share UI across routes with `layout.rs` files
- **Live reload** - Changes automatically refresh the browser
- **Tailwind CSS v4** - Utility-first CSS that scans your Rust and TSX files
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
├── layout.rs       → Wraps all pages
├── index.rs        → GET /
├── about.rs        → GET /about
└── users/
    ├── layout.rs   → Wraps /users/* pages
    ├── index.rs    → GET /users
    └── [id].rs     → GET /users/:id
```

Each route file exports a `page` function:

```rust
use rejoice::{html, Markup};

pub async fn page() -> Markup {
    html! {
        h1 { "Hello, world!" }
    }
}
```

## Layouts

Layouts wrap pages and nested layouts. Create a `layout.rs` file to share UI:

```rust
use rejoice::{html, Children, Markup, DOCTYPE};

pub async fn layout(children: Children) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head { title { "My App" } }
            body {
                nav { a href="/" { "Home" } }
                main { (children) }
                footer { "Built with Rejoice" }
            }
        }
    }
}
```

Layouts nest automatically. A page at `/users/123` will be wrapped by:
1. `routes/layout.rs` (if exists)
2. `routes/users/layout.rs` (if exists)
3. `routes/users/[id].rs`

## SolidJS Islands

Add interactive components to your pages:

```rust
use rejoice::{html, island, Markup};

pub async fn page() -> Markup {
    html! {
        h1 { "My Page" }
        (island!(Counter, { initial: 0 }))
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

## Tailwind CSS

Tailwind CSS v4 is included out of the box. Just use Tailwind classes in your Rust templates or TSX components:

```rust
use rejoice::{html, Markup};

pub async fn page() -> Markup {
    html! {
        h1 class="text-4xl font-bold text-blue-600" { "Hello!" }
        p class="mt-4 text-gray-700" { "Styled with Tailwind." }
    }
}
```

Tailwind automatically scans your `src/**/*.rs` and `client/**/*.tsx` files for classes.
