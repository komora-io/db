# CLAUDE.md - Guidance for Agentic Coding Assistants

## Build & Test Commands
* Build: `cargo build` (or `cargo build --release` for optimized build)
* Run all tests: `cargo test`
* Run specific test: `cargo test <test_name>` (e.g. `cargo test smoke_00`)
* Run tests in file: `cargo test -- --test <filename>` (e.g. `cargo test -- --test smoke`)
* Show test output: `cargo test -- --nocapture`
* Lint: `cargo clippy`

## Code Style Guidelines
* **Naming**: snake_case for functions/variables, CamelCase for types/structs
* **Imports**: Group by standard lib, external crates, then internal modules
* **Organization**: Each component in its own module file (db.rs, tx.rs, etc.)
* **Error handling**: Use `io::Result<T>` and propagate errors with `?` operator
* **Types**: Use appropriate visibility modifiers, implement relevant traits
* **Comments**: Limited documentation, focus on explaining complex logic
* **Testing**: Separate test directory with common utilities in `common` module

Follow standard Rust idioms for code structure and organization.