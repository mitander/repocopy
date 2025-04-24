# repocopy

A simple command-line tool to generate a structured XML representation of your project's files, suitable for context packing. It excludes common build artifacts, VCS directories, and other configured files/directories by default.

## Features
-   Recursively scans a project directory.
-   Generates a single XML output containing file paths and their content.
-   Excludes directories and files based on a configuration file or command-line flags.
-   Skips hidden files (files starting with `.`).
-   Copies output to clipboard by default.
-   Optionally writes output to a file.

## Installation
*Install directly using cargo:*
```bash
cargo install --git https://github.com/mitander/repocopy
```
*Or build from source:*
```bash
git clone https://github.com/mitander/repocopy
cd repocopy
cargo install --path .
```
This installs the `repocopy` binary to `~/.cargo/bin/`. Ensure this directory is in your system's `$PATH`.

## Usage
```bash
repocopy ~/my-project --exclude-dirs dist --exclude-files .env
```
## Configuration (Optional)

You can customize the default exclusions by creating a configuration file at `~/.config/repocopy/config.yaml` (create the `repocopy` directory if needed). The tool loads exclusions from this file if it exists, otherwise using built-in defaults. Command-line flags (`--exclude-dirs`, `--exclude-files`) always override the config file settings.

**Example `~/.config/repocopy/config.yaml`:**

```yaml
# List of directory names to exclude
exclude_dirs:
  - target
  - node_modules
  - .git
  - .venv

# List of exact file names to exclude
exclude_files:
  - Cargo.lock
  - package-lock.json
  - .DS_Store
```

## License
[MIT](/LICENSE)
