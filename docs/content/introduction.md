# Introduction

Rejoice is a simple and delightful web framework for Rust. It combines the best ideas from modern web development into a cohesive, productive experience:

- **File-based routing** - Drop a file in `src/routes/` and it becomes a route
- **Nested layouts** - Wrap pages with shared UI automatically
- **Type-safe templates** - Compile-time HTML with Maud
- **SolidJS islands** - Add interactivity exactly where you need it
- **Tailwind CSS v4** - Style with utility classes, zero config
- **Live reload** - Instant feedback during development

## Why Rejoice?

Rejoice is built for developers who want to build web applications quickly without sacrificing type safety or performance. It takes inspiration from frameworks like Next.js and Remix but brings them to the Rust ecosystem.

### Server-first

Everything in Rejoice is server-rendered by default. Your pages are HTML that loads fast and works without JavaScript. When you need interactivity, you add it selectively with islands.

### Type-safe all the way

From your routes to your templates, everything is checked at compile time. Catch bugs before they reach production.

### Zero configuration

Start a new project and everything just works. Tailwind scans your templates automatically. Routes are discovered from the filesystem. No config files to write.

## Quick Example

Here's a complete Rejoice page:

```rust
use rejoice::{Req, Res, html};

pub async fn page(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Hello from Rejoice!" }
        p { "This is a server-rendered page." }
    })
}
```

Save this as `src/routes/index.rs` and it becomes your homepage.

## Next Steps

- [Installation](/docs/installation) - Get Rejoice set up on your machine
- [Project Structure](/docs/project-structure) - Understand how a Rejoice app is organized
- [Routing](/docs/routing) - Learn about file-based routing
