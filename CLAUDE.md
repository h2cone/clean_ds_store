# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

`clean_ds_store` is a high-performance CLI tool written in Rust that recursively scans directories to find and safely remove `.DS_Store` files by moving them to the system trash instead of permanently deleting them. The tool is designed to be cross-platform (Windows, macOS, Linux) and safe with multiple validation layers.

## Build and Development Commands

```bash
# Build release version
cargo build --release

# Build debug version
cargo build

# Run tests
cargo test

# Run in development mode with arguments
cargo run -- --dry-run
cargo run -- --help
cargo run -- /path/to/directory --verbose

# Code quality checks
cargo clippy
cargo fmt

# Install locally
cargo install --path .
```

The compiled binary is located at:
- `target/release/clean-ds-store.exe` (Windows)
- `target/release/clean-ds-store` (Linux/macOS)

## Architecture

### Single-file Design
The entire application is contained in `src/main.rs` (~300 lines). This is intentional for simplicity.

### Core Components

1. **Args struct** (lines 15-39): CLI argument parsing using clap's derive macros
   - Defines all command-line options
   - Auto-generates help text and validation

2. **CleanStats struct** (lines 41-79): Thread-safe statistics tracking
   - Uses `Arc<AtomicUsize>` for concurrent-safe counters
   - Tracks found/moved/failed file counts

3. **main()** (lines 81-161): Entry point and orchestration
   - Validates input path
   - Displays scan configuration
   - Calls `scan_and_clean()`
   - Displays statistics summary

4. **scan_and_clean()** (lines 163-243): Core scanning logic
   - Configures `WalkDir` based on CLI args
   - Uses `filter_entry()` to skip hidden directories when requested
   - Strictly validates each file is named exactly `.DS_Store`
   - Calls `move_to_trash()` for each valid file

5. **is_ds_store_file()** (lines 245-257): Safety validation
   - Performs strict filename matching (must be exactly `.DS_Store`)
   - Called twice: during scan and before trash operation (defense in depth)

6. **move_to_trash()** (lines 259-285): Trash operation with safety checks
   - Double-validates filename
   - Checks file existence and type
   - Uses `trash::delete()` for cross-platform trash operation

### Safety Architecture

The tool implements **defense in depth** with multiple safety layers:

1. Path validation at startup (exists, is directory)
2. Filename validation during iteration (`is_ds_store_file()`)
3. Re-validation before trash operation
4. File type checking (must be file, not directory)
5. Existence check before operation
6. Trash instead of permanent deletion

This multi-layer approach prevents accidental deletion of non-`.DS_Store` files.

## Dependencies and Their Roles

- **clap** (v4.5): CLI parsing with derive macros - generates argument parser from struct
- **walkdir** (v2.5): Efficient recursive directory traversal with filtering
- **trash** (v5.2): Cross-platform trash/recycle bin operations
- **anyhow** (v1.0): Ergonomic error handling with context
- **colored** (v2.1): Terminal color output for user feedback

## Language and Localization

All code comments, documentation, and output messages MUST be in English. This is an open-source project intended for international users.

- Code comments: English only
- User-facing output (println!, eprintln!): English only
- Error messages: English only
- Documentation: English only

## Testing Strategy

The project includes unit tests for the critical safety function:
- `test_is_ds_store_file()`: Validates strict filename matching logic

When adding features, ensure safety-critical functions have corresponding tests.

## Key Design Principles

1. **Safety First**: Multiple validation layers prevent accidental deletion
2. **Trash, Don't Delete**: Always use trash operations for recoverability
3. **Clear Feedback**: Colored output distinguishes different message types
4. **Cross-Platform**: All dependencies support Windows/macOS/Linux
5. **Performance**: Efficient iteration with minimal memory allocation
