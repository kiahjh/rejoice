# HTML Templates

Rejoice uses [Maud](https://maud.lambda.xyz/) for type-safe, compile-time HTML templating.

## Basic Syntax

```rust
use rejoice::html;

html! {
    div {
        h1 { "Hello, World!" }
        p { "Welcome to my site." }
    }
}
```

Self-closing elements use a semicolon:

```rust
html! {
    img src="/logo.png" alt="Logo";
    input type="text" name="email";
}
```

## Dynamic Content

Use parentheses for Rust expressions:

```rust
let name = "Alice";
let count = 42;

html! {
    p { "Hello, " (name) "!" }
    p { "Count: " (count) }
}
```

## Conditionals and Loops

```rust
html! {
    @if is_admin {
        p { "Admin panel" }
    } @else {
        p { "User view" }
    }
    
    ul {
        @for item in &items {
            li { (item) }
        }
    }
}
```

## Components

Create reusable components as helper functions that return `Markup`:

```rust
use rejoice::{Req, Res, html, Markup};

fn card(title: &str, content: &str) -> Markup {
    html! {
        div class="card" {
            h2 { (title) }
            p { (content) }
        }
    }
}

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        (card("Welcome", "Hello!"))
        (card("About", "Learn more."))
    })
}
```

## DOCTYPE

Include `DOCTYPE` in your root layout:

```rust
use rejoice::{html, DOCTYPE};

html! {
    (DOCTYPE)
    html {
        head { title { "My App" } }
        body { /* ... */ }
    }
}
```

For complete Maud documentation, see [maud.lambda.xyz](https://maud.lambda.xyz/).
