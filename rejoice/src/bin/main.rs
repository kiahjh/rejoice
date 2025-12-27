mod commands;

use commands::style;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1).map(|s| s.as_str()) {
        Some("--version" | "-v") => style::print_version(),
        Some("--help" | "-h") => style::print_usage(),
        Some("init") => commands::init_command(args.get(2)),
        Some("dev") => commands::dev_command(),
        Some(cmd) => {
            style::print_error(&format!("Unknown command: {}", cmd));
            println!();
            style::print_usage();
            std::process::exit(1);
        }
        None => {
            style::print_usage();
        }
    }
}
