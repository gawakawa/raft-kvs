# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands
- Build: `cargo build`
- Run: `cargo run -- <port_number>` (e.g., `cargo run -- 6000`)
- Check: `cargo check`
- Test: `cargo test`
- Test specific: `cargo test <test_name>`
- Clippy lint: `cargo clippy -- -D warnings`
- Format code: `cargo fmt`

## Code Style Guidelines
- Follow Rust 2024 edition conventions
- Use `serde` for serialization/deserialization with `#[derive]` macros
- Error handling: Use `Result<T, E>` with `?` operator; avoid unwraps in production code
- Type annotation: Use explicit types for function signatures and struct fields
- Naming: Use snake_case for variables/functions, CamelCase for types/traits
- Comments: Add documentation comments with `///` for public items
- Imports: Group std imports first, then external crates, then local modules
- Prefer strong typing over generic JSON when possible
- Match statements should handle all possible cases explicitly