use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use futures::{SinkExt, StreamExt};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--version" | "-v") => println!("rejoice {}", env!("CARGO_PKG_VERSION")),
        Some("init") => init_command(args.get(2)),
        Some("dev") => dev_command(),
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
        None => {
            eprintln!("Usage: rejoice <command>");
            eprintln!("Commands:");
            eprintln!("  init [name]  Create a new rejoice project");
            eprintln!("  dev          Start development server");
            eprintln!("  --version    Show version");
            std::process::exit(1);
        }
    }
}

fn init_command(name: Option<&String>) {
    let project_name = name.map(|s| s.as_str()).unwrap_or("my-app");
    let project_dir = Path::new(project_name);

    if project_dir.exists() {
        eprintln!("Error: Directory '{}' already exists", project_name);
        std::process::exit(1);
    }

    println!("Creating new rejoice project: {}", project_name);

    // Create directory structure
    std::fs::create_dir_all(project_dir.join("src/routes")).expect("Failed to create directories");

    // Write Cargo.toml
    let rejoice_version = env!("CARGO_PKG_VERSION");
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
rejoice = "{}"
tokio = {{ version = "1.48.0", features = ["full"] }}

[build-dependencies]
rejoice = "{}"
"#,
        project_name, rejoice_version, rejoice_version
    );
    std::fs::write(project_dir.join("Cargo.toml"), cargo_toml).expect("Failed to write Cargo.toml");

    // Write build.rs
    let build_rs = r#"fn main() {
    rejoice::codegen::generate_routes();
}
"#;
    std::fs::write(project_dir.join("build.rs"), build_rs).expect("Failed to write build.rs");

    // Write main.rs
    let main_rs = r#"use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
"#;
    std::fs::write(project_dir.join("src/main.rs"), main_rs).expect("Failed to write main.rs");

    // Write index route
    let index_rs = r#"use axum::response::Html;

pub async fn handler() -> Html<&'static str> {
    Html("<h1>Welcome to Rejoice!</h1>")
}
"#;
    std::fs::write(project_dir.join("src/routes/index.rs"), index_rs)
        .expect("Failed to write index.rs");

    // Write .gitignore
    let gitignore = format!(
        r#"/target
.env
{}.db
"#,
        project_name
    );
    std::fs::write(project_dir.join(".gitignore"), gitignore).expect("Failed to write .gitignore");

    // Create empty sqlite database
    let db_path = project_dir.join(format!("{}.db", project_name));
    std::fs::File::create(&db_path).expect("Failed to create database file");
    let db_absolute_path = std::fs::canonicalize(&db_path).expect("Failed to get absolute path");

    // Write .env
    let dotenv = format!("DATABASE_URL=sqlite:{}\n", db_absolute_path.display());
    std::fs::write(project_dir.join(".env"), dotenv).expect("Failed to write .env");

    println!("Project created successfully!");
    println!();
    println!("To get started:");
    println!("  cd {}", project_name);
    println!("  rejoice dev");
}

fn dev_command() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(async_dev_command());
}

async fn async_dev_command() {
    println!("Starting development server...");

    // Channel for broadcasting reload signals to WebSocket clients
    let (reload_tx, _) = broadcast::channel::<()>(16);
    let reload_tx = Arc::new(reload_tx);

    // Start the WebSocket server for live reload
    let reload_tx_clone = reload_tx.clone();
    tokio::spawn(async move {
        run_reload_server(reload_tx_clone).await;
    });

    // File watcher channel
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher");

    // Watch the src directory
    let src_path = Path::new("src");
    if src_path.exists() {
        watcher
            .watch(src_path, RecursiveMode::Recursive)
            .expect("Failed to watch src directory");
        println!("Watching src/ for changes...");
    } else {
        eprintln!("No src/ directory found");
        std::process::exit(1);
    }

    // Also watch Cargo.toml
    let cargo_toml = Path::new("Cargo.toml");
    if cargo_toml.exists() {
        watcher
            .watch(cargo_toml, RecursiveMode::NonRecursive)
            .expect("Failed to watch Cargo.toml");
    }

    // Start the app
    let mut child = start_app();

    // Debounce: don't restart more than once per second
    let mut last_restart = Instant::now();
    let debounce_duration = Duration::from_secs(1);

    println!("Live reload server running on ws://localhost:3001/__reload");

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                use notify::EventKind::*;
                match event.kind {
                    Create(_) | Modify(_) | Remove(_) => {
                        if last_restart.elapsed() > debounce_duration {
                            println!("\nFile changed: {:?}", event.paths);
                            println!("Restarting...\n");

                            // Kill the old process
                            let _ = child.kill();
                            let _ = child.wait();

                            // Start a new one
                            child = start_app();
                            last_restart = Instant::now();

                            // Wait a moment for the app to start, then signal reload
                            let reload_tx = reload_tx.clone();
                            tokio::spawn(async move {
                                tokio::time::sleep(Duration::from_millis(500)).await;
                                let _ = reload_tx.send(());
                            });
                        }
                    }
                    _ => {}
                }
            }
            Ok(Err(e)) => {
                eprintln!("Watch error: {:?}", e);
            }
            Err(e) => {
                eprintln!("Channel error: {:?}", e);
                break;
            }
        }
    }
}

async fn run_reload_server(reload_tx: Arc<broadcast::Sender<()>>) {
    let app = Router::new().route(
        "/__reload",
        get(move |ws: WebSocketUpgrade| {
            let rx = reload_tx.subscribe();
            async move { ws.on_upgrade(|socket| handle_reload_socket(socket, rx)) }
        }),
    );

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3001")
        .await
        .expect("Failed to bind reload server");

    axum::serve(listener, app).await.unwrap();
}

async fn handle_reload_socket(socket: WebSocket, mut rx: broadcast::Receiver<()>) {
    let (mut sender, mut receiver) = socket.split();

    // Spawn a task to handle incoming messages (just to keep connection alive)
    tokio::spawn(async move { while let Some(Ok(_)) = receiver.next().await {} });

    // Wait for reload signals and send them to the client
    while rx.recv().await.is_ok() {
        if sender.send(Message::Text("reload".into())).await.is_err() {
            break;
        }
    }
}

fn start_app() -> Child {
    Command::new("cargo")
        .args(["run"])
        .env("REJOICE_DEV", "1")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start cargo run")
}
