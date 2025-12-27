use colored::Colorize;

pub const BANNER: &str = r#"
            _       _          
           (_)     (_)         
   _ __ ___ _  ___  _  ___ ___ 
  | '__/ _ \ |/ _ \| |/ __/ _ \
  | | |  __/ | (_) | | (_|  __/
  |_|  \___| |\___/|_|\___\___|
          _/ |                 
         |__/                  "#;

pub fn print_banner() {
    println!("{}", BANNER.bright_magenta().bold());
}

pub fn print_version() {
    println!("\n{}", "Rejoice".magenta().bold());
    println!(
        "{} {}\n",
        "version".dimmed(),
        env!("CARGO_PKG_VERSION").bright_white()
    );
}

pub fn print_usage() {
    print_banner();
    println!(
        "  {}",
        "A simple and delightful web framework for Rust".dimmed()
    );
    println!("\n{}", "USAGE:".bright_white().bold());
    println!("  rejoice {}", "<command>".cyan());
    println!("{}", "\nCOMMANDS:".bright_white().bold());
    println!(
        "  {} {}  {}",
        "init".cyan(),
        "[name]".dimmed(),
        "Create a new rejoice project".white()
    );
    println!(
        "  {}          {}",
        "dev".cyan(),
        "Start the development server".white()
    );
    println!("{}", "\nOPTIONS:".bright_white().bold());
    println!(
        "  {}   {}",
        "-v, --version".cyan(),
        "Show version information".white()
    );
    println!(
        "  {}       {}\n",
        "-h, --help".cyan(),
        "Show this help message".white()
    );
}

pub fn print_error(msg: &str) {
    eprintln!("{} {}", "error:".red().bold(), msg);
}

pub fn print_success(msg: &str) {
    println!("{} {}", "✓".green().bold(), msg);
}

pub fn print_step(step: usize, total: usize, msg: &str) {
    println!(
        "{} {}",
        format!("[{}/{}]", step, total).dimmed(),
        msg.white()
    );
}

pub fn print_compiling() {
    println!("{} {}", "↻".yellow().bold(), "Compiling...".yellow());
}
