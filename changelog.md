# Changelog

## 0.12.2

- Improved error messaging when Vite build fails due to missing modules - now suggests clearing `node_modules` and reinstalling dependencies

## 0.12.1

- Fixed Windows compatibility: moved `tokio`, `tower`, and `tower-http` out of Unix-only dependencies so the crate compiles on Windows

## 0.12.0

- **Switched from npm to Bun** - All client-side builds now use Bun for faster installs and builds
- **Dynamic routes in directories** - You can now use `[param]/` directories for nested dynamic routes (e.g., `users/[id]/posts/` → `/users/:id/posts`)
- **Multiple dynamic segments** - Routes can have multiple path parameters (e.g., `/users/:user_id/posts/:post_id`)
- **Database migrations CLI** - New `rejoice migrate` command with `add`, `up`, `revert`, and `status` subcommands (wraps sqlx-cli)
- **Auto-generated boilerplate** - When the dev server is running, creating a new route or layout file auto-populates it with appropriate boilerplate
- **Fixed dev server cleanup** - Ctrl+C now properly kills child processes, preventing "address already in use" errors
- **llms.txt support** - Documentation now follows the llmstxt.org standard for better AI agent integration

## 0.11.4

- Fixed npm PATH resolution issues - now runs npm commands through shell to properly detect npm installed via nvm or custom locations

## 0.11.3

- Improved error messages when `npm install` fails during `rejoice dev`

## 0.11.2

- Fixed Windows compatibility: route codegen now uses forward slashes in `#[path]` attributes

## 0.11.1

- Added `query_scalar` to database exports for COUNT, MAX, and other single-value queries
- Fixed `Res` chaining: finalizer methods (`html`, `json`, `redirect`, etc.) now take `&self` instead of `self`, allowing patterns like `res.delete_cookie("x").redirect("/y")`

## 0.11.0

- Added support for other http methods (GET, POST, PUT, DELETE, PATCH) and body parsing

## 0.10.1

- Route names with underscores like `foo_bar.rs` now get converted to `/foo-bar`

## 0.10.0

- Added `Req` type for read-only access to incoming request data (headers, cookies, method, uri)
- Added `Res` type for building responses with `set_*` mutators and finalizers (`html`, `json`, `redirect`, `raw`)
- Routes can now return redirects, JSON, or raw responses in addition to HTML
- Non-HTML responses bypass layout wrapping automatically
- Simplified route signatures: state is now passed as a plain value instead of `State<T>` wrapper
- Flattened exports: `html!`, `Markup`, `DOCTYPE`, `PreEscaped` are now available at root level
- Added `prelude` module for convenient imports

## 0.9.2

- Made sqlx an optional dependency behind the "sqlite" feature flag

## 0.9.1

- No JS by default; if there's no islands in `client/`, no JS gets sent to the client

## 0.9.0

- Added `rejoice build` command for smoother deployment

## 0.8.0

- `public/` directory for static assets served at root URL path

## 0.7.2

- Fixed import issues causing build errors

## 0.7.1

- Now if you run `rejoice init` with no name for a project, it will use the current directory name and initialize a project at `.`

## 0.7.0

- App state
- SQLite support (optional, passed to routes through state)

## 0.6.3

- Tweaked --version output

## 0.6.2

- Improved CLI output

## 0.6.1

- Re-export `Path` from `axum::extract`

## 0.6.0

- Added support for nestable layouts

## 0.5.0

- Tailwind CSS support
- Improved live reloading

## 0.4.2

- Made prop syntax not require quotes around keys

## 0.4.1

- Fixed lsp for TypeScript files in `client/` dir

## 0.4.0

- Island architecture using SolidJS

## 0.3.0

- Added Maud for HTML templating

## 0.2.2

- Less verbose output for `rejoice dev`

## 0.2.1

- Added `rejoice --version` or `rejoice -v` command

## 0.2.0

- LSP support for route files

## 0.1.0

- Initial release
