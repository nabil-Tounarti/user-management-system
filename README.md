# User Management System

A Rust workspace project demonstrating two workspaces: a user management library and a CLI application.

## Project Structure

```
├── Cargo.toml          # Root workspace configuration
├── user-lib/           # Library workspace
│   ├── Cargo.toml
│   └── src/
│       └── lib.rs
├── user-cli/           # CLI application workspace
│   ├── Cargo.toml
│   └── src/
│       └── main.rs
└── README.md
```

## Workspaces

### 1. `user-lib` - Core Library
A library crate that provides:
- User struct with validation
- UserManager for CRUD operations
- Persistence (save/load from JSON)
- Search functionality
- Comprehensive error handling

### 2. `user-cli` - CLI Application  
A binary crate that provides a command-line interface using the `user-lib`:
- Add, list, get, update, and remove users
- Search users by name
- Persistent storage in JSON format
- Clean CLI interface with clap

## Building and Running

### Build everything:
```bash
cargo build
```

### Run tests:
```bash
cargo test
```

### Run the CLI application:
```bash
# Build and run
cargo run -p user-cli -- --help

# Examples:
cargo run -p user-cli -- add -n "John Doe" -e "john@example.com" -a 30
cargo run -p user-cli -- list
cargo run -p user-cli -- get -i 1
cargo run -p user-cli -- search -q "john"
cargo run -p user-cli -- update -i 1 --age 31
cargo run -p user-cli -- remove -i 1
```

### Use custom data file:
```bash
cargo run -p user-cli -- -f my_users.json list
```

## Features

- **Workspace dependencies**: Shared dependencies defined at the workspace level
- **Clean separation**: Library logic separate from CLI interface
- **Error handling**: Comprehensive error handling with `anyhow`
- **Serialization**: JSON persistence with `serde`
- **CLI parsing**: Modern CLI with `clap` derive macros
- **Input validation**: Email and age validation
- **Search**: Case-insensitive name search
- **Testing**: Unit tests for core functionality

## Workspace Benefits

1. **Code Reusability**: The `user-lib` can be used by other applications
2. **Shared Dependencies**: Common dependencies managed at workspace level
3. **Consistent Versioning**: Easy to keep related crates in sync
4. **Build Efficiency**: Cargo can optimize builds across the workspace
5. **Development Workflow**: Easy to work on related crates together
