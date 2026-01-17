use clap::{CommandFactory, Parser, Subcommand};

mod commands;

#[derive(Parser)]
#[command(name = "rejoice")]
#[command(about = "A simple and delightful little web framework for Rust")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new Rejoice project
    Init {
        /// Project name
        name: Option<String>,
        /// Set up SQLite database with sqlx
        #[arg(long)]
        with_db: bool,
    },
    /// Start the development server
    Dev,
    /// Build the project for deployment
    Build {
        /// Build in release mode with optimizations
        #[arg(long)]
        release: bool,
    },
    /// Database migrations
    Migrate {
        #[command(subcommand)]
        action: MigrateAction,
    },
}

#[derive(Subcommand)]
enum MigrateAction {
    /// Create a new migration
    Add {
        /// Migration name (e.g., "create_users_table")
        name: String,
    },
    /// Apply pending migrations
    Up,
    /// Revert the last migration
    Revert,
    /// Show migration status
    Status,
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Some(Commands::Init { name, with_db }) => {
            commands::init_command(name.as_ref(), with_db);
        }
        Some(Commands::Dev) => {
            commands::dev_command();
        }
        Some(Commands::Build { release }) => {
            commands::build_command(release);
        }
        Some(Commands::Migrate { action }) => match action {
            MigrateAction::Add { name } => commands::migrate_add(&name),
            MigrateAction::Up => commands::migrate_up(),
            MigrateAction::Revert => commands::migrate_revert(),
            MigrateAction::Status => commands::migrate_status(),
        },
        None => {
            Cli::command().print_help().unwrap();
        }
    }
}
