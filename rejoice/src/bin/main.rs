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
    std::fs::create_dir_all(project_dir.join("client")).expect("Failed to create client directory");

    // Write Cargo.toml
    let rejoice_version = env!("CARGO_PKG_VERSION");
    let cargo_toml = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
axum = "0.8"
maud = {{ version = "0.27", features = ["axum"] }}
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

    // Write index route with island example
    let index_rs = r#"use rejoice::{html, island, Markup, DOCTYPE};

pub async fn handler() -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome" }
            }
            body {
                h1 { "Welcome to Rejoice!" }
                p { "Click the button below - it's a SolidJS island!" }
                (island!(Counter, { "initial": 0 }))
            }
        }
    }
}
"#;
    std::fs::write(project_dir.join("src/routes/index.rs"), index_rs)
        .expect("Failed to write index.rs");

    // Write package.json
    let package_json = r#"{
  "name": "app",
  "private": true,
  "type": "module",
  "scripts": {
    "build": "vite build",
    "dev": "vite build --watch"
  },
  "dependencies": {
    "solid-js": "^1.9.5"
  },
  "devDependencies": {
    "vite": "^6.3.5",
    "vite-plugin-solid": "^2.11.6"
  }
}
"#;
    std::fs::write(project_dir.join("package.json"), package_json)
        .expect("Failed to write package.json");

    // Write vite.config.ts
    let vite_config = r#"import { defineConfig } from "vite";
import solid from "vite-plugin-solid";

export default defineConfig({
  plugins: [solid()],
  build: {
    outDir: "dist",
    lib: {
      entry: "client/islands.tsx",
      name: "islands",
      fileName: () => "islands.js",
      formats: ["es"],
    },
    rollupOptions: {
      output: {
        inlineDynamicImports: true,
      },
    },
  },
});
"#;
    std::fs::write(project_dir.join("vite.config.ts"), vite_config)
        .expect("Failed to write vite.config.ts");

    // Write example Counter component
    let counter_tsx = r#"import { createSignal } from "solid-js";

interface CounterProps {
  initial: number;
}

export default function Counter(props: CounterProps) {
  const [count, setCount] = createSignal(props.initial);

  return (
    <button onClick={() => setCount((c) => c + 1)}>
      Count: {count()}
    </button>
  );
}
"#;
    std::fs::write(project_dir.join("client/Counter.tsx"), counter_tsx)
        .expect("Failed to write Counter.tsx");

    // Write tsconfig.json
    let tsconfig = r#"{
  "compilerOptions": {
    "target": "ESNext",
    "module": "ESNext",
    "moduleResolution": "node",
    "strict": true,
    "jsx": "preserve",
    "jsxImportSource": "solid-js"
  },
  "include": ["client/**/*"]
}
"#;
    std::fs::write(project_dir.join("tsconfig.json"), tsconfig)
        .expect("Failed to write tsconfig.json");

    // Write .gitignore
    let gitignore = format!(
        r#"/target
/node_modules
/dist
/client/islands.tsx
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
    // Check if client/ directory exists and set up JS build
    let client_dir = Path::new("client");
    let has_islands = client_dir.exists();
    let mut vite_child: Option<Child> = None;

    if has_islands {
        // Generate islands.tsx from client/*.tsx files
        generate_islands_registry();

        // Check if node_modules exists, if not run npm install
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

        // Start vite build in watch mode
        println!("Starting Vite build...");
        vite_child = Some(
            Command::new("npm")
                .args(["run", "dev"])
                .stdout(Stdio::null())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("Failed to start vite"),
        );

        // Give vite a moment to do initial build
        tokio::time::sleep(Duration::from_millis(500)).await;
    }

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

    // Watch client/ directory for island changes
    if has_islands {
        watcher
            .watch(client_dir, RecursiveMode::Recursive)
            .expect("Failed to watch client directory");
    }

    // Start the app
    println!("Compiling...");
    let mut child = start_app();

    // Debounce: don't restart more than once per second
    let mut last_restart = Instant::now();
    let debounce_duration = Duration::from_secs(1);

    loop {
        match rx.recv() {
            Ok(Ok(event)) => {
                use notify::EventKind::*;
                match event.kind {
                    Create(_) | Modify(_) | Remove(_) => {
                        if last_restart.elapsed() > debounce_duration {
                            // Check if it's a client file change
                            let is_client_change = event.paths.iter().any(|p| {
                                p.to_string_lossy().contains("/client/")
                                    || p.to_string_lossy().contains("\\client\\")
                            });

                            if is_client_change && has_islands {
                                // Regenerate islands registry if a new component was added
                                generate_islands_registry();
                            }

                            println!("Recompiling...");

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

    // Cleanup vite process
    if let Some(mut vite) = vite_child {
        let _ = vite.kill();
    }
}

fn generate_islands_registry() {
    let client_dir = Path::new("client");
    let Ok(entries) = std::fs::read_dir(client_dir) else {
        return;
    };

    let mut components: Vec<String> = Vec::new();

    for entry in entries.flatten() {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext == "tsx" || ext == "jsx" {
                if let Some(stem) = path.file_stem() {
                    let name = stem.to_string_lossy().to_string();
                    // Skip the islands.tsx file itself
                    if name != "islands" {
                        components.push(name);
                    }
                }
            }
        }
    }

    if components.is_empty() {
        return;
    }

    // Generate islands.tsx
    let mut output = String::new();
    output.push_str("import { render } from \"solid-js/web\";\n\n");

    // Import all components
    for name in &components {
        output.push_str(&format!("import {} from \"./{name}\";\n", name));
    }

    output.push_str("\nconst islands: Record<string, any> = {\n");
    for name in &components {
        output.push_str(&format!("  {},\n", name));
    }
    output.push_str("};\n\n");

    output.push_str(
        r#"function hydrateIslands() {
  document.querySelectorAll("[data-island]").forEach((el) => {
    const name = el.getAttribute("data-island");
    const props = JSON.parse(el.getAttribute("data-props") || "{}");
    const Component = islands[name!];
    if (Component) {
      render(() => <Component {...props} />, el);
    }
  });
}

if (document.readyState === "loading") {
  document.addEventListener("DOMContentLoaded", hydrateIslands);
} else {
  hydrateIslands();
}
"#,
    );

    std::fs::write(client_dir.join("islands.tsx"), output).expect("Failed to write islands.tsx");
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
        .args(["run", "--quiet"])
        .env("REJOICE_DEV", "1")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn()
        .expect("Failed to start cargo run")
}
