use anyhow::Result;
use clap::{Parser, Subcommand};
use user_lib::UserManager;

#[derive(Parser)]
#[command(name = "user-cli")]
#[command(about = "A CLI tool for managing users")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    #[arg(short, long, default_value = "users.json")]
    file: String,
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
            if users.is_empty() {
                println!("No users found.");
            } else {
                println!("{:<4} {:<20} {:<30} {:<4}", "ID", "Name", "Email", "Age");
                println!("{}", "-".repeat(60));
                for user in users {
                    println!("{:<4} {:<20} {:<30} {:<4}", user.id, user.name, user.email, user.age);
                }
            }
        }
        Commands::Get { id } => {
            match manager.get_user(id) {
                Some(user) => {
                    println!("User found:");
                    println!("ID: {}", user.id);
                    println!("Name: {}", user.name);
                    println!("Email: {}", user.email);
                    println!("Age: {}", user.age);
                }
                None => println!("User with ID {} not found.", id),
            }
        }
        Commands::Remove { id } => {
            match manager.remove_user(id) {
                Ok(user) => {
                    println!("Removed user: {} ({})", user.name, user.email);
                    manager.save_to_file(&cli.file)?;
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Commands::Update { id, name, email, age } => {
            match manager.update_user(id, name, email, age) {
                Ok(()) => {
                    println!("Updated user with ID: {}", id);
                    manager.save_to_file(&cli.file)?;
                }
                Err(e) => println!("Error: {}", e),
            }
        }
        Commands::Search { query } => {
            let users = manager.search_by_name(&query);
            if users.is_empty() {
                println!("No users found matching '{}'.", query);
            } else {
                println!("Found {} user(s) matching '{}':", users.len(), query);
                println!("{:<4} {:<20} {:<30} {:<4}", "ID", "Name", "Email", "Age");
                println!("{}", "-".repeat(60));
                for user in users {
                    println!("{:<4} {:<20} {:<30} {:<4}", user.id, user.name, user.email, user.age);
                }
            }
        }
    }

    Ok(())
}
