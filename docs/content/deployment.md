# Deployment

Deploy your Rejoice app to production.

## Building for Production

```bash
rejoice build --release
```

This creates an optimized binary and compiled assets.

## Required Files

Your deployment needs these files:

```text
my-app/
├── target/release/my-app   # The compiled binary
├── dist/                   # Built client assets
│   ├── islands.js
│   └── styles.css
├── public/                 # Static files
└── .env                    # Environment variables (if using database)
```

## Running the Binary

The binary must run from the project root (where `dist/` and `public/` exist):

```bash
cd /path/to/my-app
./target/release/my-app
```

Or copy everything to a deployment directory:

```bash
mkdir deploy
cp target/release/my-app deploy/
cp -r dist deploy/
cp -r public deploy/
cp .env deploy/        # if using database
cd deploy
./my-app
```

## Environment Variables

### Compile-time Variables

Variables read with `rejoice::env!()` are embedded at compile time:

```rust
let db_url = rejoice::env!("DATABASE_URL");
```

These require recompilation to change.

### Runtime Variables

For runtime configuration, use `std::env`:

```rust
let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap();

let app = App::new(port, create_router());
```

## Database

If using SQLite:

1. Ensure the `.db` file exists at the path in `DATABASE_URL`
2. The file needs read/write permissions
3. Consider absolute paths in production

## Reverse Proxy

In production, put Rejoice behind a reverse proxy like nginx or Caddy.

**Nginx example:**

```nginx
server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

## Systemd Service

Create a systemd service for auto-restart:

**`/etc/systemd/system/my-app.service`**:

```ini
[Unit]
Description=My Rejoice App
After=network.target

[Service]
Type=simple
User=www-data
WorkingDirectory=/var/www/my-app
ExecStart=/var/www/my-app/my-app
Restart=always
RestartSec=5
Environment="DATABASE_URL=sqlite:./my-app.db"

[Install]
WantedBy=multi-user.target
```

Enable and start:

```bash
sudo systemctl enable my-app
sudo systemctl start my-app
```

## Docker

Example `Dockerfile`:

```dockerfile
FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p my-app

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/my-app .
COPY dist/ dist/
COPY public/ public/
EXPOSE 8080
CMD ["./my-app"]
```

## Checklist

Before deploying:

- Build with `--release` flag
- Copy `dist/` directory
- Copy `public/` directory
- Set up environment variables
- Configure reverse proxy
- Set up process manager (systemd, Docker, etc.)
- Configure firewall (only expose 80/443)

## Next Steps

- [CLI Commands](/docs/cli) - Build command options
- [Database](/docs/database) - Production database setup
