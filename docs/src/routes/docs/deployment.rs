use crate::markdown::code_block_with_filename;
use rejoice::{html, island, json, Req, Res};

pub async fn get(req: Req, res: Res) -> Res {
    let _ = req;

    let deployment_tree = json!([
        {
            "name": "my-app/",
            "type": "folder",
            "children": [
                {
                    "name": "target/release/",
                    "type": "folder",
                    "children": [
                        { "name": "my-app", "type": "file", "comment": "The compiled binary" }
                    ]
                },
                {
                    "name": "dist/",
                    "type": "folder",
                    "comment": "Built client assets",
                    "children": [
                        { "name": "islands.js", "type": "file" },
                        { "name": "styles.css", "type": "file" }
                    ]
                },
                {
                    "name": "public/",
                    "type": "folder",
                    "comment": "Static files",
                    "children": []
                },
                { "name": ".env", "type": "file", "comment": "Environment variables (if using database)" }
            ]
        }
    ]);

    res.html(html! {
        h1 { "Deployment" }

        p { "Deploy your Rejoice app to production." }

        h2 { "Building for Production" }

        (code_block_with_filename("rejoice build --release", "bash", None))

        p { "This creates an optimized binary and compiled assets." }

        h2 { "Required Files" }

        p { "Your deployment needs these files:" }

        (island!(FileTree, { items: deployment_tree }))

        h2 { "Running the Binary" }

        p {
            "The binary must run from the project root (where " code { "dist/" }
            " and " code { "public/" } " exist):"
        }

        (code_block_with_filename(r#"cd /path/to/my-app
./target/release/my-app"#, "bash", None))

        p { "Or copy everything to a deployment directory:" }

        (code_block_with_filename(r#"mkdir deploy
cp target/release/my-app deploy/
cp -r dist deploy/
cp -r public deploy/
cp .env deploy/        # if using database
cd deploy
./my-app"#, "bash", None))

        h2 { "Environment Variables" }

        h3 { "Compile-time Variables" }

        p {
            "Variables read with " code { "rejoice::env!()" } " are embedded at compile time:"
        }

        (code_block_with_filename(r#"let db_url = rejoice::env!("DATABASE_URL");"#, "rust", None))

        p { "These require recompilation to change." }

        h3 { "Runtime Variables" }

        p { "For runtime configuration, use " code { "std::env" } ":" }

        (code_block_with_filename(r#"let port: u16 = std::env::var("PORT")
    .unwrap_or_else(|_| "8080".to_string())
    .parse()
    .unwrap();

let app = App::new(port, create_router());"#, "rust", None))

        h2 { "Database" }

        p { "If using SQLite:" }

        ol {
            li { "Ensure the " code { ".db" } " file exists at the path in " code { "DATABASE_URL" } }
            li { "The file needs read/write permissions" }
            li { "Consider absolute paths in production" }
        }

        h2 { "Reverse Proxy" }

        p { "In production, put Rejoice behind a reverse proxy like nginx or Caddy." }

        p { strong { "Nginx example:" } }

        (code_block_with_filename(r#"server {
    listen 80;
    server_name example.com;

    location / {
        proxy_pass http://127.0.0.1:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}"#, "nginx", None))

        h2 { "Systemd Service" }

        p { "Create a systemd service for auto-restart:" }

        (code_block_with_filename(r#"[Unit]
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
WantedBy=multi-user.target"#, "ini", Some("/etc/systemd/system/my-app.service")))

        p { "Enable and start:" }

        (code_block_with_filename(r#"sudo systemctl enable my-app
sudo systemctl start my-app"#, "bash", None))

        h2 { "Docker" }

        p { "Example " code { "Dockerfile" } ":" }

        (code_block_with_filename(r#"FROM rust:1.85 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release -p my-app

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/my-app .
COPY dist/ dist/
COPY public/ public/
EXPOSE 8080
CMD ["./my-app"]"#, "dockerfile", Some("Dockerfile")))

        h2 { "Checklist" }

        p { "Before deploying:" }

        ul {
            li { "Build with " code { "--release" } " flag" }
            li { "Copy " code { "dist/" } " directory" }
            li { "Copy " code { "public/" } " directory" }
            li { "Set up environment variables" }
            li { "Configure reverse proxy" }
            li { "Set up process manager (systemd, Docker, etc.)" }
            li { "Configure firewall (only expose 80/443)" }
        }

        h2 { "Next Steps" }

        ul {
            li { a href="/docs/cli" { "CLI Commands" } " — Build command options" }
            li { a href="/docs/database" { "Database" } " — Production database setup" }
        }
    })
}
