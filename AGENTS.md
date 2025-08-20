# AGENTS.md

This repository is a Rust project (no-std-ish vibe) with a simple lexer (scanner) in `src/lexer` and a small `main.rs` entry.

## Build / Lint / Test
- Build: `cargo build` or `cargo build --release`.
- Run tests: `cargo test` (single test: `cargo test -- <FILTER>` or `cargo test -- <pattern>`).
- Run a specific test function: `cargo test <.module::TestName>`.
- Lint: `cargo clippy --all-targets -- -D warnings`.
- Format: `cargo fmt`.

## Code Style Guidelines
- Imports: group std, crates, then super/crate items; use `use` paths clearly.
- Formatting: 4 spaces; trailing commas where helpful; maximum line length ~100.
- Types: prefer explicit types; use `Result<T, E>` with `anyhow`-style errors when appropriate; propagate errors with `?`.
- Naming: snake_case for functions/vars, CamelCase for types/traits, ALL_CAPS for constants.
- Error handling: use `Result`/`Option`; map errors with `?` or `bail!` for early exits; avoid unwraps in library code.
- Public API: `pub` items documented (`///`) and unit-tested.
- Tests: unit tests in adjacent modules; integration tests in `tests/` when present.

## Cursor / Copilot Rules
- Cursor: adhere to current cursor rules in `.cursor/rules/` if present.
- Copilot: follow instructions in `.github/copilot-instructions.md` if present.
