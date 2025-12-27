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
