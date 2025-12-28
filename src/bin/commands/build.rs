use super::islands::generate_islands_registry;
use super::style;
use colored::Colorize;
use std::path::Path;
use std::process::{Command, Stdio};

pub fn build_command(release: bool) {
    style::print_banner();

    let mode = if release { "release" } else { "debug" };
    println!(
        "\n  {}\n",
        format!("Building for {}...", mode).dimmed()
    );

    let client_dir = Path::new("client");
    let has_client = client_dir.exists();

    // Determine total steps
    let total_steps = if has_client { 4 } else { 1 };
    let mut step = 1;

    // Step 1: Install npm dependencies if needed
    if has_client {
        if !Path::new("node_modules").exists() {
            style::print_step(step, total_steps, "Installing npm dependencies...");
            let status = Command::new("npm")
                .args(["install"])
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .status();

            if status.is_err() || !status.unwrap().success() {
                style::print_error("Failed to run npm install");
                std::process::exit(1);
            }
        } else {
            style::print_step(step, total_steps, "npm dependencies already installed");
        }
        step += 1;

        // Step 2: Generate islands registry
        style::print_step(step, total_steps, "Generating islands registry...");
        generate_islands_registry();
        step += 1;

        // Step 3: Build client assets with Vite
        style::print_step(step, total_steps, "Building client assets...");
        let vite_status = Command::new("npm")
            .args(["run", "build"])
            .stdout(Stdio::null())
            .stderr(Stdio::inherit())
            .status();

        if vite_status.is_err() || !vite_status.unwrap().success() {
            style::print_error("Failed to build client assets");
            std::process::exit(1);
        }
        step += 1;
    }

    // Step 4 (or 1): Build Rust binary
    let build_msg = if release {
        "Building Rust binary (release)..."
    } else {
        "Building Rust binary (debug)..."
    };
    style::print_step(step, total_steps, build_msg);

    let mut cargo_args = vec!["build"];
    if release {
        cargo_args.push("--release");
    }

    let cargo_status = Command::new("cargo")
        .args(&cargo_args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    if cargo_status.is_err() || !cargo_status.unwrap().success() {
        style::print_error("Failed to build Rust binary");
        std::process::exit(1);
    }

    // Print success message
    println!();
    style::print_success(&format!("Build complete! ({})", mode));

    // Print deployment instructions for release builds
    if release {
        print_deployment_info(has_client);
    }
}

fn print_deployment_info(_has_client: bool) {
    let binary_name = get_project_name().unwrap_or_else(|| "your-app".to_string());

    println!();
    println!("{}", "  To run:".white().bold());
    println!("    ./target/release/{}", binary_name);
    println!();
    println!(
        "  {}",
        "Or clone this repo on your server and run the binary from the project root.".dimmed()
    );
}

fn get_project_name() -> Option<String> {
    let cargo_toml = std::fs::read_to_string("Cargo.toml").ok()?;
    for line in cargo_toml.lines() {
        if line.starts_with("name") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() == 2 {
                return Some(
                    parts[1]
                        .trim()
                        .trim_matches('"')
                        .trim_matches('\'')
                        .to_string(),
                );
            }
        }
    }
    None
}
