# Tailwind CSS

Rejoice includes [Tailwind CSS v4](https://tailwindcss.com/docs) with zero configuration.

## Usage

Add classes directly in your templates:

```rust
html! {
    div class="container mx-auto px-4" {
        h1 class="text-4xl font-bold text-blue-600" { "Welcome!" }
        button class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600" {
            "Click me"
        }
    }
}
```

In SolidJS islands, use `class` (not `className`):

```tsx
export default function Card(props: { title: string }) {
  return (
    <div class="bg-white shadow-lg rounded-lg p-6">
      <h2 class="text-xl font-semibold">{props.title}</h2>
    </div>
  );
}
```

## Configuration

Tailwind is configured in `client/styles.css`:

```css
@import "tailwindcss";

@source "../src/**/*.rs";
@source "./**/*.tsx";
```

The `@source` directives tell Tailwind to scan your Rust and TSX files for class names.

## Custom Styles

Add custom CSS after the imports:

```css
@import "tailwindcss";

@source "../src/**/*.rs";
@source "./**/*.tsx";

/* Your custom styles */
.custom-gradient {
  background: linear-gradient(to right, #3b82f6, #8b5cf6);
}
```

For complete Tailwind documentation, see [tailwindcss.com/docs](https://tailwindcss.com/docs).
