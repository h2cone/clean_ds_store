use anyhow::{Context, Result};
use clap::Parser;
use colored::*;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use walkdir::WalkDir;

/// High-performance CLI tool to recursively clean .DS_Store junk files
///
/// This tool recursively scans the specified directory, finds all .DS_Store files,
/// and safely moves them to the system trash (instead of permanently deleting them)
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Directory path to scan (defaults to current directory)
    #[arg(default_value = ".")]
    path: PathBuf,

    /// Preview mode: only show files that would be removed, without actually executing
    #[arg(short = 'n', long = "dry-run")]
    dry_run: bool,

    /// Verbose output: display each file found
    #[arg(short, long)]
    verbose: bool,

    /// Do not recursively scan subdirectories
    #[arg(long = "no-recursive")]
    no_recursive: bool,

    /// Maximum recursion depth (0 means unlimited)
    #[arg(long, default_value = "0")]
    max_depth: usize,

    /// Skip hidden directories (directories starting with ., but not .DS_Store files)
    #[arg(long)]
    skip_hidden: bool,
}

struct CleanStats {
    found: Arc<AtomicUsize>,
    moved: Arc<AtomicUsize>,
    failed: Arc<AtomicUsize>,
}

impl CleanStats {
    fn new() -> Self {
        Self {
            found: Arc::new(AtomicUsize::new(0)),
            moved: Arc::new(AtomicUsize::new(0)),
            failed: Arc::new(AtomicUsize::new(0)),
        }
    }

    fn increment_found(&self) {
        self.found.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_moved(&self) {
        self.moved.fetch_add(1, Ordering::Relaxed);
    }

    fn increment_failed(&self) {
        self.failed.fetch_add(1, Ordering::Relaxed);
    }

    fn get_found(&self) -> usize {
        self.found.load(Ordering::Relaxed)
    }

    fn get_moved(&self) -> usize {
        self.moved.load(Ordering::Relaxed)
    }

    fn get_failed(&self) -> usize {
        self.failed.load(Ordering::Relaxed)
    }
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Validate input path
    let scan_path = args
        .path
        .canonicalize()
        .context("Cannot access the specified path")?;

    if !scan_path.exists() {
        anyhow::bail!("Path does not exist: {}", scan_path.display());
    }

    if !scan_path.is_dir() {
        anyhow::bail!("Path is not a directory: {}", scan_path.display());
    }

    // Display scan information
    println!(
        "{} {}",
        "Scan path:".bold().cyan(),
        scan_path.display().to_string().yellow()
    );

    if args.dry_run {
        println!(
            "{}",
            "Mode: Preview mode (files will not be removed)"
                .bold()
                .yellow()
        );
    } else {
        println!(
            "{}",
            "Mode: Execution mode (files will be moved to trash)"
                .bold()
                .green()
        );
    }

    if args.no_recursive {
        println!("{}", "Recursion: Disabled".bold());
    } else if args.max_depth > 0 {
        println!("{} {}", "Max depth:".bold(), args.max_depth);
    }

    println!();

    // Start scanning
    let stats = CleanStats::new();
    scan_and_clean(&scan_path, &args, &stats)?;

    // Display statistics
    println!();
    println!("{}", "=".repeat(50).bright_black());
    println!("{}", "Cleanup Statistics:".bold().cyan());
    println!(
        "  {} {}",
        "Found .DS_Store files:".bold(),
        stats.get_found().to_string().yellow()
    );

    if !args.dry_run {
        println!(
            "  {} {}",
            "Successfully moved to trash:".bold(),
            stats.get_moved().to_string().green()
        );
        if stats.get_failed() > 0 {
            println!(
                "  {} {}",
                "Failed:".bold(),
                stats.get_failed().to_string().red()
            );
        }
    }
    println!("{}", "=".repeat(50).bright_black());

    if args.dry_run && stats.get_found() > 0 {
        println!();
        println!(
            "{}",
            "Tip: Remove --dry-run flag to actually execute cleanup".bright_yellow()
        );
    }

    Ok(())
}

fn scan_and_clean(path: &Path, args: &Args, stats: &CleanStats) -> Result<()> {
    let mut walker = WalkDir::new(path);

    // Configure walker
    if args.no_recursive {
        walker = walker.max_depth(1);
    } else if args.max_depth > 0 {
        walker = walker.max_depth(args.max_depth);
    }

    // Iterate through directories
    for entry in walker.into_iter().filter_entry(|e| {
        // Filter conditions
        if args.skip_hidden && e.depth() > 0 {
            // Skip hidden directories (but not root directory and files)
            if e.file_type().is_dir() {
                if let Some(name) = e.file_name().to_str() {
                    if name.starts_with('.') {
                        return false;
                    }
                }
            }
        }
        true
    }) {
        match entry {
            Ok(entry) => {
                // Check if it's a file
                if !entry.file_type().is_file() {
                    continue;
                }

                // Strictly check if filename is .DS_Store
                if !is_ds_store_file(&entry.path()) {
                    continue;
                }

                stats.increment_found();

                let file_path = entry.path();

                if args.verbose || args.dry_run {
                    if args.dry_run {
                        println!("{} {}", "[Preview]".bright_yellow(), file_path.display());
                    } else {
                        println!("{} {}", "[Found]".bright_blue(), file_path.display());
                    }
                }

                // If not in preview mode, move to trash
                if !args.dry_run {
                    match move_to_trash(file_path) {
                        Ok(_) => {
                            stats.increment_moved();
                            if args.verbose {
                                println!("  {} {}", "✓".green().bold(), "Moved to trash".green());
                            }
                        }
                        Err(e) => {
                            stats.increment_failed();
                            eprintln!(
                                "  {} Failed to move file: {}",
                                "✗".red().bold(),
                                e.to_string().red()
                            );
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("{} {}", "Warning:".yellow(), e);
            }
        }
    }

    Ok(())
}

/// Strictly check if the filename is .DS_Store
///
/// This function ensures we only process genuine .DS_Store files,
/// avoiding accidental deletion of other files
fn is_ds_store_file(path: &Path) -> bool {
    if let Some(file_name) = path.file_name() {
        if let Some(name_str) = file_name.to_str() {
            // Strict filename matching
            return name_str == ".DS_Store";
        }
    }
    false
}

/// Safely move files to the system trash
///
/// Uses the trash crate to ensure cross-platform compatibility
fn move_to_trash(path: &Path) -> Result<()> {
    // Verify filename again (double safety check)
    if !is_ds_store_file(path) {
        anyhow::bail!(
            "Safety check failed: filename is not .DS_Store: {}",
            path.display()
        );
    }

    // Confirm file exists
    if !path.exists() {
        anyhow::bail!("File does not exist: {}", path.display());
    }

    // Confirm it's a file and not a directory
    if !path.is_file() {
        anyhow::bail!("Not a file: {}", path.display());
    }

    // Move to trash
    trash::delete(path).context(format!("Failed to move to trash: {}", path.display()))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_ds_store_file() {
        assert!(is_ds_store_file(Path::new("/some/path/.DS_Store")));
        assert!(is_ds_store_file(Path::new(".DS_Store")));
        assert!(!is_ds_store_file(Path::new("/some/path/DS_Store")));
        assert!(!is_ds_store_file(Path::new("/some/path/.DS_Store.txt")));
        assert!(!is_ds_store_file(Path::new("/some/path/file.txt")));
        assert!(!is_ds_store_file(Path::new("/some/path/.DS_Store2")));
    }
}
