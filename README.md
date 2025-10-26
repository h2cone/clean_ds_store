# Clean DS_Store

A high-performance Rust CLI tool to recursively clean `.DS_Store` junk files.

## Features

- **Safe and Reliable**: Only deletes `.DS_Store` files with strict filename validation to prevent accidental deletion
- **Trash Instead of Delete**: Moves files to system trash instead of permanently deleting them, allowing recovery
- **High Performance**: Written in Rust with efficient recursive directory traversal
- **Preview Mode**: Supports dry-run mode to preview before execution
- **Cross-Platform**: Supports Windows, macOS, and Linux
- **Rich Options**: Supports depth control, hidden directory skipping, and more

## Installation

### Build from Source

```bash
# Clone or navigate to the project directory
cd clean_ds_store

# Build the project
cargo build --release

# The executable will be located at:
# target/release/clean-ds-store (Linux/macOS)
# target\release\clean-ds-store.exe (Windows)
```

### Install to System

```bash
cargo install --path .
```

## Usage

### Basic Usage

```bash
# Clean current directory (preview mode)
clean-ds-store --dry-run

# Clean current directory (actual execution)
clean-ds-store

# Clean specific directory
clean-ds-store /path/to/directory

# Verbose output
clean-ds-store --verbose

# Or use short form
clean-ds-store -v
```

### Advanced Options

```bash
# Preview mode (only show files that would be removed, without actual deletion)
clean-ds-store --dry-run
clean-ds-store -n

# Limit recursion depth
clean-ds-store --max-depth 3

# Do not recurse into subdirectories (current directory only)
clean-ds-store --no-recursive

# Skip hidden directories
clean-ds-store --skip-hidden

# Combine options
clean-ds-store /path/to/dir --verbose --max-depth 5 --skip-hidden
```

### View Help

```bash
clean-ds-store --help
```

## Examples

### Example 1: Preview Cleanup

```bash
$ clean-ds-store --dry-run
Scan path: /Users/username/projects
Mode: Preview mode (files will not be removed)

[Preview] /Users/username/projects/.DS_Store
[Preview] /Users/username/projects/subfolder/.DS_Store

==================================================
Cleanup Statistics:
  Found .DS_Store files: 2
==================================================

Tip: Remove --dry-run flag to actually execute cleanup
```

### Example 2: Execute Cleanup

```bash
$ clean-ds-store --verbose
Scan path: /Users/username/projects
Mode: Execution mode (files will be moved to trash)

[Found] /Users/username/projects/.DS_Store
  ✓ Moved to trash
[Found] /Users/username/projects/subfolder/.DS_Store
  ✓ Moved to trash

==================================================
Cleanup Statistics:
  Found .DS_Store files: 2
  Successfully moved to trash: 2
==================================================
```

## Security

This tool implements multiple safety measures:

1. **Strict Filename Validation**: Only processes files with exact name match of `.DS_Store`
2. **Double Check**: Verifies filename again before moving to trash
3. **Type Checking**: Ensures only files are processed, not directories
4. **Existence Check**: Confirms file exists before operation
5. **Trash Instead of Delete**: Files can be recovered from trash instead of being permanently deleted

## Performance

- Uses `walkdir` crate for efficient directory traversal
- Atomic operation statistics for concurrent safety
- Minimal memory footprint
- Zero-copy path handling

## Tech Stack

- **Rust** - Systems programming language
- **clap** - Command-line argument parsing
- **walkdir** - Efficient recursive directory traversal
- **trash** - Cross-platform trash/recycle bin operations
- **anyhow** - Error handling
- **colored** - Colored terminal output

## Development

### Run Tests

```bash
cargo test
```

### Run in Development Mode

```bash
cargo run -- --dry-run
cargo run -- --help
```

### Code Checking

```bash
cargo clippy
cargo fmt
```

## FAQ

**Q: Will files be permanently deleted?**
A: No. All files are moved to the system trash and can be recovered at any time.

**Q: How do I recover accidentally removed files?**
A: Recover them from the system trash (Windows Recycle Bin, macOS Trash, Linux Trash).

**Q: Which platforms are supported?**
A: Windows, macOS, and Linux are supported.

**Q: How is the performance?**
A: Written in Rust with excellent performance. Can quickly scan thousands of directories in large projects.

**Q: Why not delete files directly?**
A: For safety reasons, moving to trash prevents data loss from accidental operations.

## License

MIT License

## Contributing

Issues and Pull Requests are welcome!
