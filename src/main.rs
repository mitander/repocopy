use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const EXCLUDE_DIRS: &[&str] = &["target", ".git", "node_modules"];

fn should_skip_path(path: &Path) -> bool {
    // Check if path contains excluded directory
    if let Some(path_str) = path.to_str() {
        if EXCLUDE_DIRS.iter().any(|dir| path_str.contains(dir)) {
            return true;
        }
    }

    // Check if file is hidden
    if let Some(file_name) = path.file_name() {
        if let Some(file_str) = file_name.to_str() {
            if file_str.starts_with('.') {
                return true;
            }
        }
    }

    false
}

fn process_directory(root_path: &Path) -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();

    writeln!(handle, "<documents>")?;
    let mut index = 1;

    for entry in WalkDir::new(root_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories and excluded paths
        if path.is_dir() || should_skip_path(path) {
            continue;
        }

        // Get relative path
        let relative_path = path
            .strip_prefix(root_path)
            .unwrap_or(path)
            .to_string_lossy();

        // Read file content
        let content = match fs::read_to_string(path) {
            Ok(content) => content,
            Err(e) => {
                eprintln!("Error reading {}: {}", relative_path, e);
                continue;
            }
        };

        // Write document
        writeln!(handle, "<document index=\"{index}\">")?;
        writeln!(handle, "<source>{}</source>", relative_path)?;
        writeln!(handle, "<document_content>")?;
        write!(handle, "{}", content)?;
        writeln!(handle, "</document_content>")?;
        writeln!(handle, "</document>")?;

        index += 1;
    }

    writeln!(handle, "</documents>")?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <directory_path>", args[0]);
        std::process::exit(1);
    }

    let path = PathBuf::from(&args[1]);
    if !path.exists() {
        eprintln!("Directory not found: {}", args[1]);
        std::process::exit(1);
    }

    process_directory(&path)
}
