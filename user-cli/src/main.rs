use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand, ValueEnum};
use user_lib::UserManager;

#[derive(Parser)]
#[command(name = "user-cli")]
#[command(about = "A CLI tool for managing users")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "users.json")]
    file: String,

    #[arg(short = 'F', long = "format", value_enum, default_value_t = OutputFormat::Table)]
    format: OutputFormat,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
enum OutputFormat {
    Table,
    Json,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new user
    Add {
        #[arg(short, long)]
        name: String,
        #[arg(short, long)]
        email: String,
        #[arg(short, long)]
        age: u8,
    },
    /// List all users
    List,
    /// Get a specific user by ID
    Get {
        #[arg(short, long)]
        id: u32,
    },
    /// Remove a user by ID
    Remove {
        #[arg(short, long)]
        id: u32,
    },
    /// Update a user
    Update {
        #[arg(short, long)]
        id: u32,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        email: Option<String>,
        #[arg(long)]
        age: Option<u8>,
    },
    /// Search users by name
    Search {
        #[arg(short, long)]
        query: String,
    },
}

fn print_single_user(user: &user_lib::User, format: OutputFormat) -> Result<()> {
    match format {
        OutputFormat::Table => {
            println!("{:<4} {:<20} {:<30} {:<4}", "ID", "Name", "Email", "Age");
            println!("{}", "-".repeat(60));
            println!(
                "{:<4} {:<20} {:<30} {:<4}",
                user.id, user.name, user.email, user.age
            );
            Ok(())
        }
        OutputFormat::Json => {
            let json = serde_json::to_string_pretty(user)?;
            println!("{}", json);
            Ok(())
        }
    }
}

fn print_users(mut users: Vec<&user_lib::User>, format: OutputFormat) -> Result<()> {
    // Sort by id ascending for stable output
    users.sort_by_key(|u| u.id);
    match format {
        OutputFormat::Table => {
            if users.is_empty() {
                println!("No users found.");
                return Ok(());
            }
            println!("{:<4} {:<20} {:<30} {:<4}", "ID", "Name", "Email", "Age");
            println!("{}", "-".repeat(60));
            for user in users {
                println!(
                    "{:<4} {:<20} {:<30} {:<4}",
                    user.id, user.name, user.email, user.age
                );
            }
            Ok(())
        }
        OutputFormat::Json => {
            // Serialize owned copies for JSON
            let owned: Vec<user_lib::User> = users.into_iter().cloned().collect();
            let json = serde_json::to_string_pretty(&owned)?;
            println!("{}", json);
            Ok(())
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut manager = UserManager::new();

    // Try to load existing data
    if std::path::Path::new(&cli.file).exists() {
        manager.load_from_file(&cli.file)?;
    }

    match cli.command {
        Commands::Add { name, email, age } => {
            let id = manager.add_user(name, email, age)?;
            println!("Added user with ID: {}", id);
            manager.save_to_file(&cli.file)?;
        }
        Commands::List => {
            let users = manager.list_users();
            print_users(users, cli.format)?;
        }
        Commands::Get { id } => {
            match manager.get_user(id) {
                Some(user) => {
                    print_single_user(user, cli.format)?;
                }
                None => eprintln!("User with ID {} not found.", id),
            }
        }
        Commands::Remove { id } => {
            match manager.remove_user(id) {
                Ok(user) => {
                    println!("Removed user: {} ({})", user.name, user.email);
                    manager.save_to_file(&cli.file)?;
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Update { id, name, email, age } => {
            if name.is_none() && email.is_none() && age.is_none() {
                return Err(anyhow!("At least one of --name, --email, --age must be provided"));
            }
            match manager.update_user(id, name, email, age) {
                Ok(()) => {
                    println!("Updated user with ID: {}", id);
                    manager.save_to_file(&cli.file)?;
                }
                Err(e) => eprintln!("Error: {}", e),
            }
        }
        Commands::Search { query } => {
            let users = manager.search_by_name(&query);
            if users.is_empty() {
                println!("No users found matching '{}'.", query);
            } else {
                print_users(users, cli.format)?;
            }
        }
    }

    Ok(())
}
