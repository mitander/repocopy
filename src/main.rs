use std::env;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use walkdir::WalkDir; // for writing to String

const EXCLUDE_DIRS: &[&str] = &["target", ".git", "node_modules"];
const EXCLUDE_FILES: &[&str] = &["Cargo.lock"];

fn should_skip_path(path: &Path) -> bool {
    // Check if the path contains any excluded directory.
    if let Some(path_str) = path.to_str() {
        if EXCLUDE_DIRS.iter().any(|dir| path_str.contains(dir)) {
            return true;
        }
    }

    // Check file-specific exclusions.
    if let Some(file_name) = path.file_name() {
        if let Some(file_str) = file_name.to_str() {
            // Exclude specific files.
            if EXCLUDE_FILES.iter().any(|&excluded| excluded == file_str) {
                return true;
            }
            // Exclude hidden files.
            if file_str.starts_with('.') {
                return true;
            }
        }
    }

    false
}

fn generate_xml(root_path: &Path) -> io::Result<String> {
    let mut output = String::new();
    writeln!(&mut output, "<documents>").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut index = 1;

    for entry in WalkDir::new(root_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        // Skip directories and any paths that should be excluded.
        if path.is_dir() || should_skip_path(path) {
            continue;
        }

        let relative_path = path
            .strip_prefix(root_path)
            .unwrap_or(path)
            .to_string_lossy();

        let content = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!("Error reading {}: {}", relative_path, e);
                continue;
            }
        };

        writeln!(&mut output, "<document index=\"{index}\">")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "<source>{}</source>", relative_path)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "<document_content>")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        write!(&mut output, "{}", content).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "</document_content>")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "</document>")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        index += 1;
    }

    writeln!(&mut output, "</documents>").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(output)
}

fn main() -> io::Result<()> {
    // Basic argument parsing: required directory path and an optional "-f <output_file>" flag.
    let mut args = env::args().skip(1);
    let mut directory: Option<PathBuf> = None;
    let mut output_file: Option<PathBuf> = None;

    while let Some(arg) = args.next() {
        if arg == "-f" {
            if let Some(file_name) = args.next() {
                output_file = Some(PathBuf::from(file_name));
            } else {
                eprintln!("Error: -f flag provided but no output file specified.");
                std::process::exit(1);
            }
        } else if directory.is_none() {
            directory = Some(PathBuf::from(arg));
        }
    }

    let dir = match directory {
        Some(d) => d,
        None => {
            eprintln!("Usage: <program> <directory_path> [-f <output_file>]");
            std::process::exit(1);
        }
    };

    if !dir.exists() {
        eprintln!("Directory not found: {:?}", dir);
        std::process::exit(1);
    }

    // Generate the XML content.
    let xml_content = generate_xml(&dir)?;

    if let Some(file_path) = output_file {
        fs::write(&file_path, xml_content)?;
        println!("Output written to file: {:?}", file_path);
    } else {
        // Copy the content to the clipboard using the arboard crate.
        let mut clipboard =
            arboard::Clipboard::new().map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        clipboard
            .set_text(xml_content)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        println!("Output copied to clipboard.");
    }

    Ok(())
}
