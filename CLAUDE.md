# CLAUDE.md - Project Guidelines for adsb-rust-dataset

## Project Overview

ADS-B data collection utility that reads aircraft surveillance data from dump1090 (SBS-1 protocol over TCP) and forwards it to SentinelOne DataSet (Scalyr) for logging and analysis.

## Build & Development Commands

```bash
# Build
cargo build                    # Debug build
cargo build --release          # Release build

# Run
cargo run -- --DUMP1090_HOST=localhost --DUMP1090_PORT=30003 --DATASET_API_WRITE_TOKEN=<token>

# Test
cargo test                     # Run all tests
cargo test -- --nocapture      # Run tests with stdout

# Linting & Formatting
cargo fmt                      # Format code
cargo fmt -- --check           # Check formatting (CI)
cargo clippy                   # Run lints
cargo clippy -- -D warnings    # Fail on any warning (CI)

# Full check before commit
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test
```

## Code Style & Idioms

### Rust Edition
- Use Rust 2021 edition features

### Formatting
- Always run `cargo fmt` before committing
- Use default rustfmt settings (no rustfmt.toml overrides unless necessary)

### Linting (Clippy)
Enable these lints in `main.rs` or `lib.rs`:
```rust
#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
```

Key lint expectations:
- No `unwrap()` in library code; use `?` or explicit error handling
- Prefer `expect("reason")` over `unwrap()` when panicking is intentional
- Use `if let` or `match` instead of `.unwrap()` for Options in non-critical paths

### Error Handling
- Define custom error types with `thiserror` for library code
- Use `anyhow` for application-level error handling
- Propagate errors with `?` operator; avoid silent failures
- Log errors before dropping them

### Naming Conventions
- Types: `PascalCase` (e.g., `SBS1Message`)
- Functions/methods: `snake_case` (e.g., `parse_message`)
- Constants: `SCREAMING_SNAKE_CASE`
- Modules: `snake_case`

### Idiomatic Patterns

**Prefer iterators over manual loops:**
```rust
// Good
let values: Vec<_> = items.iter().filter(|x| x.is_valid()).collect();

// Avoid
let mut values = Vec::new();
for item in &items {
    if item.is_valid() {
        values.push(item);
    }
}
```

**Use `Option` combinators:**
```rust
// Good
let name = input.as_ref().map(|s| s.trim().to_string());

// Avoid
let name = match input {
    Some(s) => Some(s.trim().to_string()),
    None => None,
};
```

**Prefer `impl Trait` for return types when appropriate:**
```rust
fn get_items() -> impl Iterator<Item = String> { ... }
```

**Use struct update syntax:**
```rust
let updated = Config { batch_size: 1000, ..default_config };
```

### Documentation
- Add `///` doc comments to public types and functions
- Include examples in doc comments for complex APIs
- Use `//` for implementation comments (sparingly)

### Dependencies
- Pin major versions in Cargo.toml (e.g., `serde = "1.0"` not `serde = "*"`)
- Prefer well-maintained crates from the ecosystem
- Minimize dependency count; evaluate if stdlib suffices

## Project Structure

```
src/
├── main.rs     # Entry point, TCP handling, HTTP client, batching logic
└── parse.rs    # SBS-1 message parsing (SBS1Message struct and parser)
```

### Module Responsibilities

- **main.rs**: Configuration, async runtime, TCP stream reading, message batching, DataSet API communication
- **parse.rs**: SBS-1 protocol parsing, message struct definition, field type conversions

## Testing Guidelines

- Place unit tests in the same file using `#[cfg(test)]` module
- Use `#[test]` attribute for test functions
- Name tests descriptively: `test_parse_valid_msg3_with_position`
- Test edge cases: empty fields, malformed input, boundary values

Example test structure:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_message() {
        let input = "MSG,3,...";
        let result = parse(input);
        assert!(result.is_some());
    }
}
```

## Architecture Notes

### Data Flow
```
dump1090 (TCP:30003) → SBS-1 Parser → Message Queue → DataSet API (HTTPS)
```

### Key Types
- `SBS1Message`: Parsed aircraft surveillance message (ICAO, position, velocity, etc.)
- Configuration via CLI args or environment variables

### Async Runtime
- Uses Tokio for async I/O
- Main loop: read line → parse → enqueue → batch send
