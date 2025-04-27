use clap::Parser;
use quick_xml::escape::escape;
use std::fmt::Write as FmtWrite;
use std::fs;
use std::io;
use std::path::{Component, Path, PathBuf};
use walkdir::WalkDir;

mod config;
use config::{load_config, Config};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The root directory of the project to scan.
    directory: PathBuf,

    /// Write output to a file instead of the clipboard.
    #[arg(short, long)]
    output_file: Option<PathBuf>,

    /// Comma-separated list of directories to exclude (overrides config).
    #[arg(long)]
    exclude_dirs: Option<String>,

    /// Comma-separated list of files to exclude (overrides config).
    #[arg(long)]
    exclude_files: Option<String>,
}

fn should_skip_path(path: &Path, exclusions: &Config) -> bool {
    // Check against excluded directory names in path components
    if path.components().any(|component| {
        if let Component::Normal(name) = component {
            if let Some(name_str) = name.to_str() {
                return exclusions
                    .exclude_dirs
                    .iter()
                    .any(|excluded| *excluded == name_str);
            }
        }
        false
    }) {
        return true;
    }

    if let Some(file_name) = path.file_name() {
        if let Some(file_str) = file_name.to_str() {
            // Exclude specific files by exact match
            if exclusions
                .exclude_files
                .iter()
                .any(|excluded| *excluded == file_str)
            {
                return true;
            }
        }
    }

    false
}

fn generate_xml(root_path: &Path, exclusions: &Config) -> io::Result<String> {
    let mut output = String::new();
    writeln!(&mut output, "<documents>").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    let mut index = 1;

    for entry in WalkDir::new(root_path)
        .follow_links(true)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        let path = entry.path();

        if path.is_dir() || should_skip_path(path, exclusions) {
            continue;
        }

        let relative_path = path
            .strip_prefix(root_path)
            .unwrap_or(path)
            .to_string_lossy();

        let content = match fs::read_to_string(path) {
            Ok(text) => text,
            Err(e) => {
                eprintln!(
                    "Warning: Skipping file {} due to read error: {}",
                    relative_path, e
                );
                continue;
            }
        };

        writeln!(&mut output, "  <document index=\"{index}\">")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(
            &mut output,
            "    <source>{}</source>",
            escape(&relative_path)
        )
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "    <document_content>")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        write!(&mut output, "{}", escape(&content))
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "\n    </document_content>")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
        writeln!(&mut output, "  </document>")
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        index += 1;
    }

    writeln!(&mut output, "</documents>").map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    Ok(output)
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    if !args.directory.exists() {
        eprintln!("Error: Directory not found: {:?}", args.directory);
        std::process::exit(1);
    }
    if !args.directory.is_dir() {
        eprintln!(
            "Error: Provided path is not a directory: {:?}",
            args.directory
        );
        std::process::exit(1);
    }

    let mut config = load_config();

    if let Some(dirs_str) = args.exclude_dirs {
        config.exclude_dirs = dirs_str.split(',').map(|s| s.trim().to_string()).collect();
        println!(
            "Using command-line excluded directories: {:?}",
            config.exclude_dirs
        );
    }
    if let Some(files_str) = args.exclude_files {
        config.exclude_files = files_str.split(',').map(|s| s.trim().to_string()).collect();
        println!(
            "Using command-line excluded files: {:?}",
            config.exclude_files
        );
    }

    let xml_content = generate_xml(&args.directory, &config)?;

    if let Some(file_path) = args.output_file {
        fs::write(&file_path, xml_content)?;
        println!("Output written to file: {:?}", file_path);
    } else {
        match arboard::Clipboard::new() {
            Ok(mut clipboard) => match clipboard.set_text(xml_content) {
                Ok(_) => println!("Output copied to clipboard."),
                Err(e) => {
                    eprintln!(
                        "Error copying to clipboard: {}. Increase verbosity for details.",
                        e
                    );
                    std::process::exit(1);
                }
            },
            Err(e) => {
                eprintln!("Error initializing clipboard: {}. Cannot copy output.", e);
                std::process::exit(1);
            }
        }
    }

    Ok(())
}
