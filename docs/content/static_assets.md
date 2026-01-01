# Static Assets

The `public/` directory serves static files at the root URL path.

## File Mapping

| File Path | URL |
|-----------|-----|
| `public/logo.png` | `/logo.png` |
| `public/images/hero.jpg` | `/images/hero.jpg` |
| `public/favicon.ico` | `/favicon.ico` |
| `public/fonts/custom.woff2` | `/fonts/custom.woff2` |

## Usage in Templates

Reference static assets with absolute paths:

```rust
html! {
    head {
        link rel="icon" href="/favicon.ico";
        link rel="stylesheet" href="/fonts/fonts.css";
    }
    body {
        img src="/logo.png" alt="Logo";
        img src="/images/hero.jpg" alt="Hero image";
    }
}
```

## Route Priority

Defined routes take precedence over static files. If you have both:

- `src/routes/about.rs` 
- `public/about.html`

The route wins, and `/about` serves the Rust route.

## Built Assets

Client assets (JavaScript, CSS) are output to `dist/` and served automatically:

| Asset | URL |
|-------|-----|
| `dist/islands.js` | Auto-injected |
| `dist/styles.css` | Auto-injected |

You don't need to reference these manually—they're injected into all HTML responses.

## Common Static Files

Typical `public/` contents:

```text
public/
├── favicon.ico
├── robots.txt
├── sitemap.xml
├── images/
│   ├── logo.png
│   └── og-image.jpg
└── fonts/
    ├── inter.woff2
    └── fonts.css
```

## Custom Fonts

Add custom fonts to `public/fonts/`:

**`public/fonts/fonts.css`**:

```css
@font-face {
  font-family: 'Inter';
  src: url('/fonts/inter.woff2') format('woff2');
  font-weight: 400;
  font-style: normal;
  font-display: swap;
}
```

Reference in your layout:

```rust
html! {
    head {
        link rel="stylesheet" href="/fonts/fonts.css";
    }
    body class="font-['Inter']" {
        // Content with custom font
    }
}
```

## Development

During `rejoice dev`, the `public/` directory is watched. Changes to static files trigger a browser reload.

## Production

In production, include the `public/` directory alongside your binary:

```text
my-app/
├── my-app          # Binary
├── dist/           # Built client assets
└── public/         # Static files
```

## Next Steps

- [Tailwind CSS](/docs/tailwind) - Style your pages
- [Deployment](/docs/deployment) - Production setup
