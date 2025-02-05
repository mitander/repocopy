# repocopy

A simple and efficient command-line tool written in Rust that generates a structured XML documentation of your project's files. This tool is particularly useful for creating a comprehensive overview of your project's codebase while excluding specific directories and files.

## Features

- Recursively walks through all files in the specified directory
- Generates XML-formatted output with file paths and contents
- Excludes common unnecessary directories (target, .git, node_modules)
- Skips hidden files
- Handles symbolic links
- Efficient buffered writing to stdout

## Installation

1. Clone the repository:
```bash
git clone https://github.com/mitander/repocopy
cd repocopy
```

2. Build the project:
```bash
cargo build --release
```

The executable will be available in `target/release/`.

## Usage

```bash
cargo run -- /path/to/your/project > project_files.txt
```

Or if using the compiled binary:
```bash
./project-doc-generator /path/to/your/project > project_files.txt
```

### Output Format

The tool generates XML-formatted output like this:
```xml
<documents>
  <document index="1">
    <source>src/main.rs</source>
    <document_content>
      // File content here
    </document_content>
  </document>
  <!-- More documents... -->
</documents>
```

### Excluded Directories

By default, the following directories are excluded:
- `target/`
- `.git/`
- `node_modules/`

## Configuration

To modify the excluded directories, edit the `EXCLUDE_DIRS` constant in `src/main.rs`.

## Development

### Prerequisites

- Rust 1.54 or higher
- Cargo

### Dependencies

- `walkdir = "2.5"` - For recursive directory traversal

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

## License

MIT License
