use super::style;
use colored::Colorize;
use std::io::{self, Write};
use std::process::{Command, Stdio};

/// Check if sqlx-cli is installed
fn is_sqlx_installed() -> bool {
    Command::new("sqlx")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

/// Prompt user to install sqlx-cli
fn prompt_install_sqlx() -> bool {
    print!(
        "\n  {} sqlx-cli is not installed. Install it now? [Y/n] ",
        "?".yellow().bold()
    );
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim().to_lowercase();

    input.is_empty() || input == "y" || input == "yes"
}

/// Install sqlx-cli
fn install_sqlx() -> bool {
    println!(
        "\n  {} {}",
        "→".blue().bold(),
        "Installing sqlx-cli...".white()
    );

    let status = Command::new("cargo")
        .args([
            "install",
            "sqlx-cli",
            "--no-default-features",
            "--features",
            "sqlite",
        ])
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) if s.success() => {
            println!();
            style::print_success("sqlx-cli installed successfully!");
            true
        }
        _ => {
            style::print_error("Failed to install sqlx-cli");
            false
        }
    }
}

/// Ensure sqlx-cli is available, prompting to install if needed
fn ensure_sqlx() -> bool {
    if is_sqlx_installed() {
        return true;
    }

    if prompt_install_sqlx() {
        install_sqlx()
    } else {
        println!(
            "\n  {} {}",
            "!".yellow().bold(),
            "sqlx-cli is required for migrations. Install it with:".dimmed()
        );
        println!(
            "    {}",
            "cargo install sqlx-cli --no-default-features --features sqlite".white()
        );
        false
    }
}

/// Run a sqlx command
fn run_sqlx(args: &[&str]) -> bool {
    let status = Command::new("sqlx")
        .args(args)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status();

    match status {
        Ok(s) => s.success(),
        Err(e) => {
            style::print_error(&format!("Failed to run sqlx: {}", e));
            false
        }
    }
}

pub fn migrate_add(name: &str) {
    style::print_banner();
    println!(
        "\n  {} {}\n",
        "Creating migration:".white(),
        name.cyan().bold()
    );

    if !ensure_sqlx() {
        std::process::exit(1);
    }

    // Create migrations directory if it doesn't exist
    if !std::path::Path::new("migrations").exists() {
        std::fs::create_dir("migrations").expect("Failed to create migrations directory");
    }

    // Use -r for reversible migrations (creates up.sql and down.sql)
    if run_sqlx(&["migrate", "add", "-r", name]) {
        println!();
        style::print_success(&format!("Migration '{}' created!", name));
        println!(
            "\n  {}",
            "Edit the migration files in migrations/ then run:".dimmed()
        );
        println!("    {} {}", "$".dimmed(), "rejoice migrate up".white());
    } else {
        std::process::exit(1);
    }
}

pub fn migrate_up() {
    style::print_banner();
    println!("\n  {}\n", "Running migrations...".white());

    if !ensure_sqlx() {
        std::process::exit(1);
    }

    if run_sqlx(&["migrate", "run"]) {
        println!();
        style::print_success("Migrations applied successfully!");
    } else {
        std::process::exit(1);
    }
}

pub fn migrate_revert() {
    style::print_banner();
    println!("\n  {}\n", "Reverting last migration...".white());

    if !ensure_sqlx() {
        std::process::exit(1);
    }

    if run_sqlx(&["migrate", "revert"]) {
        println!();
        style::print_success("Migration reverted successfully!");
    } else {
        std::process::exit(1);
    }
}

pub fn migrate_status() {
    style::print_banner();
    println!("\n  {}\n", "Migration status:".white());

    if !ensure_sqlx() {
        std::process::exit(1);
    }

    // sqlx migrate info shows pending/applied migrations
    if !run_sqlx(&["migrate", "info"]) {
        std::process::exit(1);
    }
}
