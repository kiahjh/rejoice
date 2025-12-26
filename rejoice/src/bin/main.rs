mod commands;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--version" | "-v") => println!("rejoice {}", env!("CARGO_PKG_VERSION")),
        Some("init") => commands::init_command(args.get(2)),
        Some("dev") => commands::dev_command(),
        Some(cmd) => {
            eprintln!("Unknown command: {}", cmd);
            std::process::exit(1);
        }
        None => {
            print_usage();
            std::process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("Usage: rejoice <command>");
    eprintln!("Commands:");
    eprintln!("  init [name]  Create a new rejoice project");
    eprintln!("  dev          Start development server");
    eprintln!("  --version    Show version");
}
