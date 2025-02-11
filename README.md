# repocopy

A simple and efficient command-line tool written in Rust that generates a structured XML documentation of your project's files. This tool is particularly useful for creating a comprehensive overview of your project's codebase while excluding specific directories and files.

## Features

- Recursively walks through all files in the specified directory.
- Generates XML-formatted output with file paths and contents.
- Excludes common unnecessary directories (`target`, `.git`, `node_modules`) and specific files (e.g., `Cargo.lock`).
- Skips hidden files.
- Handles symbolic links.
- Efficient buffered writing to output.
- **Clipboard Integration:** By default, the generated XML is copied to your clipboard.
- **File Output Option:** Use the `-f` flag to write the output to a specified file.

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

You can run the tool either via Cargo or using the compiled binary.

### Copy to Clipboard (Default)

If you want to copy the XML output to your clipboard:

```bash
cargo run -- /path/to/your/project
```

or using the compiled binary:

```bash
./repocopy /path/to/your/project
```

After execution, the XML content will be copied to your clipboard.

### Write Output to a File

To write the output to a file, use the `-f` flag followed by the desired output file path:

```bash
cargo run -- /path/to/your/project -f output.xml
```

or using the compiled binary:

```bash
./repocopy /path/to/your/project -f output.xml
```

## Output Format

The tool generates XML-formatted output as follows:

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

## Excluded Directories and Files

By default, the following directories are excluded:

- `target/`
- `.git/`
- `node_modules/`

Additionally, the following files are excluded:

- `Cargo.lock`

To modify these exclusions, edit the `EXCLUDE_DIRS` and `EXCLUDE_FILES` constants in `src/main.rs`.

## Development

### Prerequisites

- Rust 1.54 or higher
- Cargo

### Dependencies

- `walkdir = "2.5"` – For recursive directory traversal.
- `arboard = "1.2"` – For clipboard support.

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

---
