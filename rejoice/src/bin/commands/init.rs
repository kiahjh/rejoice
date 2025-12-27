use std::path::Path;

pub fn init_command(name: Option<&String>) {
    let project_name = name.map(|s| s.as_str()).unwrap_or("my-app");
    let project_dir = Path::new(project_name);

    if project_dir.exists() {
        eprintln!("Error: Directory '{}' already exists", project_name);
        std::process::exit(1);
    }

    println!("Creating new rejoice project: {}", project_name);

    create_directories(project_dir);
    write_cargo_toml(project_dir, project_name);
    write_build_rs(project_dir);
    write_main_rs(project_dir);
    write_layout(project_dir);
    write_index_route(project_dir);
    write_package_json(project_dir);
    write_vite_config(project_dir);
    write_styles_css(project_dir);
    write_counter_component(project_dir);
    write_tsconfig(project_dir);
    write_gitignore(project_dir, project_name);
    create_database(project_dir, project_name);

    println!("Project created successfully!");
    println!();
    println!("To get started:");
    println!("  cd {}", project_name);
    println!("  rejoice dev");
}

fn create_directories(project_dir: &Path) {
    std::fs::create_dir_all(project_dir.join("src/routes")).expect("Failed to create directories");
    std::fs::create_dir_all(project_dir.join("client")).expect("Failed to create client directory");
}

fn write_cargo_toml(project_dir: &Path, project_name: &str) {
    let rejoice_version = env!("CARGO_PKG_VERSION");
    let content = format!(
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
    std::fs::write(project_dir.join("Cargo.toml"), content).expect("Failed to write Cargo.toml");
}

fn write_build_rs(project_dir: &Path) {
    let content = r#"fn main() {
    rejoice::codegen::generate_routes();
}
"#;
    std::fs::write(project_dir.join("build.rs"), content).expect("Failed to write build.rs");
}

fn write_main_rs(project_dir: &Path) {
    let content = r#"use rejoice::App;

rejoice::routes!();

#[tokio::main]
async fn main() {
    let app = App::new(8080, create_router());
    app.run().await;
}
"#;
    std::fs::write(project_dir.join("src/main.rs"), content).expect("Failed to write main.rs");
}

fn write_index_route(project_dir: &Path) {
    let content = r#"use rejoice::{html, island, Markup};

pub async fn page() -> Markup {
    html! {
        h1 { "Welcome to Rejoice!" }
        p { "Click the button below - it's a SolidJS island!" }
        (island!(Counter, { initial: 0 }))
    }
}
"#;
    std::fs::write(project_dir.join("src/routes/index.rs"), content)
        .expect("Failed to write index.rs");
}

fn write_layout(project_dir: &Path) {
    let content = r#"use rejoice::{html, Children, Markup, DOCTYPE};

pub async fn layout(children: Children) -> Markup {
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
"#;
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
    std::fs::write(project_dir.join("package.json"), content).expect("Failed to write package.json");
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

fn write_gitignore(project_dir: &Path, project_name: &str) {
    let content = format!(
        r#"/target
/node_modules
/dist
/client/islands.tsx
.env
{}.db
"#,
        project_name
    );
    std::fs::write(project_dir.join(".gitignore"), content).expect("Failed to write .gitignore");
}

fn create_database(project_dir: &Path, project_name: &str) {
    let db_path = project_dir.join(format!("{}.db", project_name));
    std::fs::File::create(&db_path).expect("Failed to create database file");
    let db_absolute_path = std::fs::canonicalize(&db_path).expect("Failed to get absolute path");

    let dotenv = format!("DATABASE_URL=sqlite:{}\n", db_absolute_path.display());
    std::fs::write(project_dir.join(".env"), dotenv).expect("Failed to write .env");
}
