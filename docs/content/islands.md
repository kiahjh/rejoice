# SolidJS Islands

Islands are interactive SolidJS components embedded in server-rendered HTML. They let you add client-side interactivity exactly where you need it.

## How Islands Work

1. Create a TSX component in `client/`
2. Use the `island!` macro in Rust to place it
3. Rejoice renders a placeholder `<div data-island="...">`
4. Client-side JavaScript hydrates the component

## Creating an Island

Create a component in `client/`:

**`client/Counter.tsx`**:

```tsx
import { createSignal } from "solid-js";

interface CounterProps {
  initial: number;
  label?: string;
}

export default function Counter(props: CounterProps) {
  const [count, setCount] = createSignal(props.initial);

  return (
    <div class="counter">
      <span>{props.label || "Count"}: {count()}</span>
      <button onClick={() => setCount(c => c + 1)}>+</button>
      <button onClick={() => setCount(c => c - 1)}>-</button>
    </div>
  );
}
```

**Important:** Export the component as `default`.

## Using an Island

Use the `island!` macro in your Rust templates:

```rust
use rejoice::{Req, Res, html, island};

pub async fn get(req: Req, res: Res) -> Res {
    res.html(html! {
        h1 { "Interactive Counter" }
        
        // Island with props
        (island!(Counter, { initial: 0 }))
        
        // Island with multiple props
        (island!(Counter, { initial: 10, label: "Score" }))
    })
}
```

## Props Syntax

```rust
// No props
(island!(SimpleComponent))

// Single prop
(island!(Counter, { initial: 42 }))

// Multiple props
(island!(UserCard, { 
    id: 123,
    name: "Alice",
    active: true
}))

// Dynamic values
let user_id = 123;
(island!(UserCard, { id: user_id }))
```

## File Naming

- Files must be in `client/` directory
- Use `.tsx` or `.jsx` extension
- Component name matches filename: `Counter.tsx` → `Counter`
- Export as `default`
- `islands.tsx` is auto-generated (don't edit)
- `styles.css` is for Tailwind (not a component)

## Islands Registry

The file `client/islands.tsx` is automatically generated. It:

- Imports all component files
- Creates a registry mapping names to components
- Handles hydration when the page loads
- Exposes `window.__hydrateIslands()` for live reload

**Never edit this file manually.** It's regenerated on every build.

## Styling Islands

Islands can use Tailwind classes just like your Rust templates:

```tsx
export default function Card(props: { title: string }) {
  return (
    <div class="bg-white shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold">{props.title}</h2>
    </div>
  );
}
```

Tailwind scans both `src/**/*.rs` and `client/**/*.tsx` for class names.

## When to Use Islands

Use islands for:

- Interactive UI (buttons, forms, toggles)
- Client-side state (counters, carts, filters)
- Real-time updates (live data, notifications)
- Complex animations

Don't use islands for:

- Static content (use server-rendered HTML)
- Content that doesn't need interactivity
- SEO-critical content

## Server-First Philosophy

Rejoice is server-first. Start with server-rendered HTML, then add islands only where needed. This gives you:

- Fast initial page loads
- Works without JavaScript
- Better SEO
- Smaller bundle sizes

## Next Steps

- [Tailwind CSS](/docs/tailwind) - Style your islands and pages
- [Templates](/docs/templates) - Server-rendered HTML with Maud
