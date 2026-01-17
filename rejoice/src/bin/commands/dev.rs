use super::boilerplate::maybe_generate_boilerplate;
use super::islands::{generate_islands_registry, generate_vite_config, has_island_components};
use super::style;
use axum::{
    Router,
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    routing::get,
};
use colored::Colorize;
use futures::{SinkExt, StreamExt};
use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::process::{Child, Command, Stdio};
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};
use std::time::{Duration, Instant};
use tokio::sync::broadcast;

// Global to track the child process ID for signal handler
static CHILD_PID: AtomicU32 = AtomicU32::new(0);

fn run_bun_command(args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    Command::new("bun")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
}

pub fn dev_command() {
    let rt = tokio::runtime::Runtime::new().unwrap();
    rt.block_on(run_dev_server());
}

async fn run_dev_server() {
    style::print_banner();
    println!("\n  {}\n", "Starting development server...".dimmed());

    let client_dir = Path::new("client");
    let has_client = client_dir.exists();

    if has_client {
        setup_client_build();
    }

    let reload_tx = Arc::new(broadcast::channel::<&'static str>(16).0);

    // Start WebSocket server for live reload
    let reload_tx_clone = reload_tx.clone();
    tokio::spawn(async move {
        run_reload_server(reload_tx_clone).await;
    });

    // Set up file watcher
    let watcher = setup_file_watcher(has_client, client_dir);

    // Start the app
    style::print_compiling();
    let mut child = start_app();

    // Set up Ctrl+C handler to clean up child process
    let ctrl_c_handler = tokio::spawn(async {
        tokio::signal::ctrl_c().await.ok();
        cleanup_child_process();
        std::process::exit(0);
    });

    // Run the watch loop (this blocks)
    run_watch_loop(watcher, &mut child, has_client, reload_tx);

    // Clean up if we exit normally
    ctrl_c_handler.abort();
    cleanup_child_process();
}

/// Kill the child process and its descendants
fn cleanup_child_process() {
    let pid = CHILD_PID.load(Ordering::SeqCst);
    if pid == 0 {
        return;
    }

    #[cfg(unix)]
    {
        // Kill the entire process group (negative PID)
        unsafe {
            libc::kill(-(pid as i32), libc::SIGTERM);
        }
        // Give it a moment to terminate gracefully
        std::thread::sleep(Duration::from_millis(100));
        // Force kill if still running
        unsafe {
            libc::kill(-(pid as i32), libc::SIGKILL);
        }
    }

    #[cfg(windows)]
    {
        // On Windows, just kill the process (child processes may still linger)
        let _ = Command::new("taskkill")
            .args(["/PID", &pid.to_string(), "/T", "/F"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status();
    }
}

fn setup_client_build() {
    let has_islands = has_island_components();

    if has_islands {
        generate_islands_registry();
    }

    if !Path::new("node_modules").exists() {
        println!(
            "{} {}",
            "→".blue().bold(),
            "Installing dependencies...".white()
        );
        let status = run_bun_command(&["install"]);

        match status {
            Err(e) => {
                style::print_error(&format!("Failed to run bun install: {}", e));
                std::process::exit(1);
            }
            Ok(s) if !s.success() => {
                style::print_error(&format!(
                    "bun install failed with exit code {}",
                    s.code()
                        .map(|c| c.to_string())
                        .unwrap_or_else(|| "unknown".to_string())
                ));
                std::process::exit(1);
            }
            Ok(_) => {}
        }
        println!();
    }

    // Generate appropriate vite config based on whether we have islands
    generate_vite_config(has_islands);

    println!(
        "{} {}",
        "→".blue().bold(),
        "Building client assets...".white()
    );
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
        style::print_error("No src/ directory found");
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

    // Watch public directory for static assets
    let public_dir = Path::new("public");
    if public_dir.exists() {
        watcher
            .watch(public_dir, RecursiveMode::Recursive)
            .expect("Failed to watch public directory");
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
                    Create(_) => {
                        // Check for new route files and generate boilerplate
                        for path in &event.paths {
                            maybe_generate_boilerplate(path);
                        }
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
                    Modify(_) | Remove(_) => {
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
                eprintln!("{} Watch error: {:?}", "!".red().bold(), e);
            }
            Err(e) => {
                eprintln!("{} Channel error: {:?}", "!".red().bold(), e);
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

    let is_public_only_change = event.paths.iter().all(|p| {
        let path_str = p.to_string_lossy();
        path_str.contains("/public/") || path_str.contains("\\public\\")
    });

    if is_public_only_change {
        // Static assets changed, just trigger a reload (no rebuild needed)
        handle_public_change(reload_tx, last_restart);
    } else if is_client_only_change && has_islands {
        handle_client_change(reload_tx, last_restart);
    } else {
        handle_rust_change(child, has_islands, reload_tx, last_restart);
    }
}

fn handle_public_change(
    reload_tx: &Arc<broadcast::Sender<&'static str>>,
    last_restart: &mut Instant,
) {
    println!(
        "{} {}",
        "↻".cyan().bold(),
        "Static assets changed...".cyan()
    );
    *last_restart = Instant::now();
    let _ = reload_tx.send("reload");
}

fn handle_client_change(
    reload_tx: &Arc<broadcast::Sender<&'static str>>,
    last_restart: &mut Instant,
) {
    println!("{} {}", "↻".cyan().bold(), "Rebuilding client...".cyan());
    let has_islands = has_island_components();
    if has_islands {
        generate_islands_registry();
    }
    generate_vite_config(has_islands);
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
        println!("{} {}", "↻".blue().bold(), "Rebuilding assets...".blue());
        run_vite_build();
    }

    style::print_compiling();

    // Kill the old process group
    cleanup_child_process();
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
    let _ = Command::new("bun")
        .args(["run", "build"])
        .stdout(Stdio::null())
        .stderr(Stdio::inherit())
        .status();
}

fn start_app() -> Child {
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        let child = unsafe {
            Command::new("cargo")
                .args(["run", "--quiet"])
                .env("REJOICE_DEV", "1")
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .pre_exec(|| {
                    // Create a new process group so we can kill all children
                    libc::setpgid(0, 0);
                    Ok(())
                })
                .spawn()
                .expect("Failed to start cargo run")
        };
        CHILD_PID.store(child.id(), Ordering::SeqCst);
        child
    }

    #[cfg(windows)]
    {
        use std::os::windows::process::CommandExt;
        // CREATE_NEW_PROCESS_GROUP (0x00000200) allows us to send Ctrl+C to the group
        const CREATE_NEW_PROCESS_GROUP: u32 = 0x00000200;
        let child = Command::new("cargo")
            .args(["run", "--quiet"])
            .env("REJOICE_DEV", "1")
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .creation_flags(CREATE_NEW_PROCESS_GROUP)
            .spawn()
            .expect("Failed to start cargo run");
        CHILD_PID.store(child.id(), Ordering::SeqCst);
        child
    }
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

    let listener = match tokio::net::TcpListener::bind("127.0.0.1:3001").await {
        Ok(l) => l,
        Err(e) => {
            eprintln!(
                "  {} Live reload unavailable (port 3001 in use: {})",
                "!".yellow().bold(),
                e
            );
            return;
        }
    };

    let _ = axum::serve(listener, app).await;
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
