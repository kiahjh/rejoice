use super::islands::{generate_islands_registry, generate_vite_config, has_island_components};
use super::style;
use colored::Colorize;
use std::path::Path;
use std::process::{Command, Stdio};

fn run_bun_command(args: &[&str]) -> std::io::Result<std::process::ExitStatus> {
    Command::new("bun")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
}

pub fn build_command(release: bool) {
    style::print_banner();

    let mode = if release { "release" } else { "debug" };
    println!("\n  {}\n", format!("Building for {}...", mode).dimmed());

    let client_dir = Path::new("client");
    let has_client = client_dir.exists();
    let has_islands = has_island_components();

    // Determine total steps:
    // - No client dir: just cargo build (1 step)
    // - Client dir with islands: bun install, generate islands, build assets, cargo build (4 steps)
    // - Client dir without islands: bun install, build assets, cargo build (3 steps)
    let total_steps = if !has_client {
        1
    } else if has_islands {
        4
    } else {
        3
    };
    let mut step = 1;

    // Step 1: Install dependencies if needed
    if has_client {
        if !Path::new("node_modules").exists() {
            style::print_step(step, total_steps, "Installing dependencies...");
            let status = run_bun_command(&["install"]);

            if status.is_err() || !status.unwrap().success() {
                style::print_error("Failed to run bun install");
                std::process::exit(1);
            }
        } else {
            style::print_step(step, total_steps, "Dependencies already installed");
        }
        step += 1;

        // Step 2 (only if islands exist): Generate islands registry
        if has_islands {
            style::print_step(step, total_steps, "Generating islands registry...");
            generate_islands_registry();
            step += 1;
        }

        // Generate appropriate vite config based on whether we have islands
        generate_vite_config(has_islands);

        // Step 3 (or 2): Build client assets with Vite
        style::print_step(step, total_steps, "Building client assets...");

        let vite_output = Command::new("bun")
            .args(["run", "build"])
            .stdout(Stdio::null())
            .stderr(Stdio::piped())
            .output();

        match vite_output {
            Ok(out) if !out.status.success() => {
                let stderr = String::from_utf8_lossy(&out.stderr);
                // Print the actual error
                eprint!("{}", stderr);

                // Check for common dependency resolution issues
                if stderr.contains("ERR_MODULE_NOT_FOUND") || stderr.contains("Cannot find module")
                {
                    eprintln!(
                        "\n  {} {}",
                        "Hint:".yellow().bold(),
                        "This may be a corrupted node_modules. Try:".white()
                    );
                    #[cfg(windows)]
                    eprintln!(
                        "    {} {}",
                        "$".dimmed(),
                        "rmdir /s /q node_modules && del bun.lock && bun install".white()
                    );
                    #[cfg(not(windows))]
                    eprintln!(
                        "    {} {}",
                        "$".dimmed(),
                        "rm -rf node_modules bun.lock && bun install".white()
                    );
                    eprintln!();
                }
                style::print_error("Failed to build client assets");
                std::process::exit(1);
            }
            Ok(_) => {}
            Err(e) => {
                style::print_error(&format!("Failed to run vite build: {}", e));
                std::process::exit(1);
            }
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
