use super::style;
use colored::Colorize;
use std::path::Path;

pub fn init_command(name: Option<&String>, with_db: bool) {
    let (project_dir, project_name, in_cwd) = if let Some(name) = name {
        let dir = Path::new(name.as_str());
        if dir.exists() {
            style::print_error(&format!("Directory '{}' already exists", name));
            std::process::exit(1);
        }
        (dir.to_path_buf(), name.clone(), false)
    } else {
        let cwd = std::env::current_dir().expect("Failed to get current directory");
        let name = cwd
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("my-app")
            .to_string();
        (cwd, name, true)
    };

    style::print_banner();
    println!(
        "\n  {} {}\n",
        "Creating project".white(),
        project_name.cyan().bold()
    );

    let total_steps = if with_db { 13 } else { 12 };
    let mut step = 0;

    step += 1;
    style::print_step(step, total_steps, "Creating directories...");
    create_directories(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing Cargo.toml...");
    write_cargo_toml(&project_dir, &project_name, with_db);

    step += 1;
    style::print_step(step, total_steps, "Writing build.rs...");
    write_build_rs(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing src/main.rs...");
    write_main_rs(&project_dir, with_db);

    step += 1;
    style::print_step(step, total_steps, "Writing routes/layout.rs...");
    write_layout(&project_dir, with_db);

    step += 1;
    style::print_step(step, total_steps, "Writing routes/index.rs...");
    write_index_route(&project_dir, with_db);

    step += 1;
    style::print_step(step, total_steps, "Writing package.json...");
    write_package_json(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing vite.config.ts...");
    write_vite_config(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing client/styles.css...");
    write_styles_css(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing client/Counter.tsx...");
    write_counter_component(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing tsconfig.json...");
    write_tsconfig(&project_dir);

    step += 1;
    style::print_step(step, total_steps, "Writing .gitignore...");
    write_gitignore(&project_dir, &project_name, with_db);

    if with_db {
        step += 1;
        style::print_step(step, total_steps, "Setting up database...");
        create_database(&project_dir, &project_name);
    }

    println!();

    style::print_success("Project created successfully!");
    println!("\n  {}", "To get started:".dimmed());
    if !in_cwd {
        println!(
            "    {} {}",
            "$".dimmed(),
            format!("cd {}", project_name).white()
        );
    }
    println!("    {} {}\n", "$".dimmed(), "rejoice dev".white());
}

fn create_directories(project_dir: &Path) {
    std::fs::create_dir_all(project_dir.join("src/routes")).expect("Failed to create directories");
    std::fs::create_dir_all(project_dir.join("client")).expect("Failed to create client directory");
    std::fs::create_dir_all(project_dir.join("public")).expect("Failed to create public directory");
}

fn write_cargo_toml(project_dir: &Path, project_name: &str, with_db: bool) {
    let rejoice_version = env!("CARGO_PKG_VERSION");
    let rejoice_dep = if with_db {
        format!(
            r#"rejoice = {{ version = "{}", features = ["sqlite"] }}"#,
            rejoice_version
        )
    } else {
        format!(r#"rejoice = "{}""#, rejoice_version)
    };
    let content = format!(
        r#"[package]
name = "{}"
version = "0.1.0"
edition = "2024"

[dependencies]
maud = {{ version = "0.27", features = ["axum"] }}
{}
tokio = {{ version = "1.48.0", features = ["full"] }}

[build-dependencies]
rejoice = "{}"
"#,
        project_name, rejoice_dep, rejoice_version
    );
    std::fs::write(project_dir.join("Cargo.toml"), content).expect("Failed to write Cargo.toml");
}

fn write_build_rs(project_dir: &Path) {
    let content = r#"fn main() {
    rejoice::codegen::generate_routes();
}
"#;
    std::fs::write(project_dir.join("build.rs"), content).expect("Failed to write build.rs");
}

fn write_main_rs(project_dir: &Path, with_db: bool) {
    let content = if with_db {
        r#"use std::time::Duration;

use rejoice::{
    App,
    db::{Pool, PoolConfig, Sqlite, create_pool},
};

rejoice::routes!(AppState);

#[derive(Clone)]
pub struct AppState {
    pub db: Pool<Sqlite>,
}

#[tokio::main]
async fn main() {
    let pool = create_pool(PoolConfig {
        db_url: rejoice::env!("DATABASE_URL").to_string(),
        max_connections: 5,
        acquire_timeout: Duration::from_secs(3),
        idle_timeout: Duration::from_secs(60),
        max_lifetime: Duration::from_secs(1800),
    })
    .await;

    let state = AppState { db: pool };

    let app = App::with_state(8080, create_router(), state);
    app.run().await;
}
"#
    } else {
        r#"use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
"#
    };
    std::fs::write(project_dir.join("src/main.rs"), content).expect("Failed to write main.rs");
}

fn write_index_route(project_dir: &Path, with_db: bool) {
    let content = if with_db {
        r#"use crate::AppState;
use rejoice::{
    State,
    html::{Markup, html},
    island,
};

pub async fn page(State(_state): State<AppState>) -> Markup {
    html! {
        h1 { "Welcome to Rejoice!" }
        p { "Click the button below - it's a SolidJS island!" }
        (island!(Counter, { initial: 0 }))
    }
}
"#
    } else {
        r#"use rejoice::{
    State,
    html::{Markup, html},
    island,
};

pub async fn page(State(_): State<()>) -> Markup {
    html! {
        h1 { "Welcome to Rejoice!" }
        p { "Click the button below - it's a SolidJS island!" }
        (island!(Counter, { initial: 0 }))
    }
}
"#
    };
    std::fs::write(project_dir.join("src/routes/index.rs"), content)
        .expect("Failed to write index.rs");
}

fn write_layout(project_dir: &Path, with_db: bool) {
    let content = if with_db {
        r#"use crate::AppState;
use rejoice::{
    Children, State,
    html::{DOCTYPE, Markup, html},
};

pub async fn layout(State(_state): State<AppState>, children: Children) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome" }
            }
            body {
                (children)
            }
        }
    }
}
"#
    } else {
        r#"use rejoice::{
    Children, State,
    html::{DOCTYPE, Markup, html},
};

pub async fn layout(State(_): State<()>, children: Children) -> Markup {
    html! {
        (DOCTYPE)
        html {
            head {
                title { "Welcome" }
            }
            body {
                (children)
            }
        }
    }
}
"#
    };
    std::fs::write(project_dir.join("src/routes/layout.rs"), content)
        .expect("Failed to write layout.rs");
}

fn write_package_json(project_dir: &Path) {
    let content = r#"{
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
    "@tailwindcss/vite": "^4.1.7",
    "tailwindcss": "^4.1.7",
    "vite": "^6.3.5",
    "vite-plugin-solid": "^2.11.6"
  }
}
"#;
    std::fs::write(project_dir.join("package.json"), content)
        .expect("Failed to write package.json");
}

fn write_vite_config(project_dir: &Path) {
    let content = r#"import { defineConfig } from "vite";
import solid from "vite-plugin-solid";
import tailwindcss from "@tailwindcss/vite";

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  build: {
    outDir: "dist",
    rollupOptions: {
      input: {
        islands: "client/islands.tsx",
        styles: "client/styles.css",
      },
      output: {
        entryFileNames: "[name].js",
        assetFileNames: "[name].[ext]",
      },
    },
  },
});
"#;
    std::fs::write(project_dir.join("vite.config.ts"), content)
        .expect("Failed to write vite.config.ts");
}

fn write_styles_css(project_dir: &Path) {
    let content = r#"@import "tailwindcss";

@source "../src/**/*.rs";
@source "./**/*.tsx";
"#;
    std::fs::write(project_dir.join("client/styles.css"), content)
        .expect("Failed to write styles.css");
}

fn write_counter_component(project_dir: &Path) {
    let content = r#"import { createSignal } from "solid-js";

interface CounterProps {
  initial: number;
}

export default function Counter(props: CounterProps) {
  const [count, setCount] = createSignal(props.initial);

  return (
    <button
      onClick={() => setCount((c) => c + 1)}
      class="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600 transition-colors"
    >
      Count: {count()}
    </button>
  );
}
"#;
    std::fs::write(project_dir.join("client/Counter.tsx"), content)
        .expect("Failed to write Counter.tsx");
}

fn write_tsconfig(project_dir: &Path) {
    let content = r#"{
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
    std::fs::write(project_dir.join("tsconfig.json"), content)
        .expect("Failed to write tsconfig.json");
}

fn write_gitignore(project_dir: &Path, project_name: &str, with_db: bool) {
    let content = if with_db {
        format!(
            r#"/target
/node_modules
/dist
/client/islands.tsx
.env
{}.db
"#,
            project_name
        )
    } else {
        r#"/target
/node_modules
/dist
/client/islands.tsx
"#
        .to_string()
    };
    std::fs::write(project_dir.join(".gitignore"), content).expect("Failed to write .gitignore");
}

fn create_database(project_dir: &Path, project_name: &str) {
    let db_path = project_dir.join(format!("{}.db", project_name));
    std::fs::File::create(&db_path).expect("Failed to create database file");
    let db_absolute_path = std::fs::canonicalize(&db_path).expect("Failed to get absolute path");

    let dotenv = format!("DATABASE_URL=sqlite:{}\n", db_absolute_path.display());
    std::fs::write(project_dir.join(".env"), dotenv).expect("Failed to write .env");
}
