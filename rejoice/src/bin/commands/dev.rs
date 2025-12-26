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

use super::islands::generate_islands_registry;

pub fn dev_command() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run_dev_server());
}

async fn run_dev_server() {
    let client_dir = Path::new("client");
    let has_islands = client_dir.exists();

    if has_islands {
        setup_client_build();
    }

    let reload_tx = Arc::new(broadcast::channel::<&'static str>(16).0);

    // Start WebSocket server for live reload
    let reload_tx_clone = reload_tx.clone();
    tokio::spawn(async move {
        run_reload_server(reload_tx_clone).await;
    });

    // Set up file watcher
    let watcher = setup_file_watcher(has_islands, client_dir);

    // Start the app
    println!("Compiling...");
    let mut child = start_app();

    // Run the watch loop
    run_watch_loop(watcher, &mut child, has_islands, reload_tx);
}

fn setup_client_build() {
    generate_islands_registry();

    if !Path::new("node_modules").exists() {
        println!("Installing npm dependencies...");
        let status = Command::new("npm")
            .args(["install"])
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .status();

        if status.is_err() || !status.unwrap().success() {
            eprintln!("Failed to run npm install");
            std::process::exit(1);
        }
    }

    println!("Building client assets...");
    run_vite_build();
}

fn setup_file_watcher(
    has_islands: bool,
    client_dir: &Path,
) -> (
    RecommendedWatcher,
    std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,
) {
    let (tx, rx) = std::sync::mpsc::channel();
    let mut watcher =
        RecommendedWatcher::new(tx, Config::default()).expect("Failed to create watcher");

    // Watch src directory
    let src_path = Path::new("src");
    if src_path.exists() {
        watcher
            .watch(src_path, RecursiveMode::Recursive)
            .expect("Failed to watch src directory");
    } else {
        eprintln!("No src/ directory found");
        std::process::exit(1);
    }

    // Watch Cargo.toml
    let cargo_toml = Path::new("Cargo.toml");
    if cargo_toml.exists() {
        watcher
            .watch(cargo_toml, RecursiveMode::NonRecursive)
            .expect("Failed to watch Cargo.toml");
    }

    // Watch client directory
    if has_islands {
        watcher
            .watch(client_dir, RecursiveMode::Recursive)
            .expect("Failed to watch client directory");
    }

    (watcher, rx)
}

fn run_watch_loop(
    _watcher: (
        RecommendedWatcher,
        std::sync::mpsc::Receiver<Result<notify::Event, notify::Error>>,
    ),
    child: &mut Child,
    has_islands: bool,
    reload_tx: Arc<broadcast::Sender<&'static str>>,
) {
    let (_watcher, rx) = _watcher;
    let mut last_restart = Instant::now();
    let debounce_duration = Duration::from_secs(1);

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                use notify::EventKind::*;
                match event.kind {
                    Create(_) | Modify(_) | Remove(_) => {
                        if last_restart.elapsed() > debounce_duration {
                            handle_file_change(
                                &event,
                                child,
                                has_islands,
                                &reload_tx,
                                &mut last_restart,
                            );
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

fn handle_file_change(
    event: &notify::Event,
    child: &mut Child,
    has_islands: bool,
    reload_tx: &Arc<broadcast::Sender<&'static str>>,
    last_restart: &mut Instant,
) {
    let is_client_only_change = event.paths.iter().all(|p| {
        let path_str = p.to_string_lossy();
        path_str.contains("/client/") || path_str.contains("\\client\\")
    });

    if is_client_only_change && has_islands {
        handle_client_change(reload_tx, last_restart);
    } else {
        handle_rust_change(child, has_islands, reload_tx, last_restart);
    }
}

fn handle_client_change(
    reload_tx: &Arc<broadcast::Sender<&'static str>>,
    last_restart: &mut Instant,
) {
    println!("Rebuilding client assets...");
    generate_islands_registry();
    run_vite_build();
    *last_restart = Instant::now();
    let _ = reload_tx.send("full");
}

fn handle_rust_change(
    child: &mut Child,
    has_islands: bool,
    reload_tx: &Arc<broadcast::Sender<&'static str>>,
    last_restart: &mut Instant,
) {
    if has_islands {
        println!("Rebuilding assets...");
        run_vite_build();
    }

    println!("Recompiling...");

    let _ = child.kill();
    let _ = child.wait();

    *child = start_app();
    *last_restart = Instant::now();

    let reload_tx = reload_tx.clone();
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(500)).await;
        let _ = reload_tx.send("reload");
    });
}

fn run_vite_build() {
    let _ = Command::new("npm")
        .args(["run", "build"])
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status();
}

fn start_app() -> Child {
    Command::new("cargo")
        .args(["run", "--quiet"])
        .env("REJOICE_DEV", "1")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start cargo run")
}

// WebSocket reload server

async fn run_reload_server(reload_tx: Arc<broadcast::Sender<&'static str>>) {
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

async fn handle_reload_socket(socket: WebSocket, mut rx: broadcast::Receiver<&'static str>) {
    let (mut sender, mut receiver) = socket.split();

    tokio::spawn(async move { while let Some(Ok(_)) = receiver.next().await {} });

    while let Ok(msg) = rx.recv().await {
        if sender.send(Message::Text(msg.into())).await.is_err() {
            break;
        }
    }
}
