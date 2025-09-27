Of course! Here is a README for your project.

# Contextualize All Project Files (capf)

`capf` is a command-line tool written in Rust that recursively finds files in a directory, reads their content, and concatenates them into a single output file. It is designed to be a fast and efficient way to create a single "context" file from a project's source code, which can be useful for analysis, documentation, or providing context to Large Language Models (LLMs).

The tool is highly configurable, allowing for fine-grained control over which files and directories are included in the output through various filtering options.

## Features

- **Recursive File Traversal**: Scans the specified input directory and all its subdirectories.
- **Multithreaded Processing**: Utilizes multiple threads to read files concurrently for improved performance.
- **Advanced Filtering**:
    - Whitelist and blacklist files based on extensions or regex patterns.
    - Ignore specific directories and files by name or regex patterns.
- **Customizable Output**: Option to include the name of each file in the output for better context.
- **Verbose Logging**: Enable detailed logging for debugging purposes.

## Installation

You can build and install the tool from the source code using Cargo.

1. **Clone the repository:**

```shell script
git clone <your-repository-url>
    cd contextualize-all-project-files
```

2. **Install using Cargo:**

```shell script
cargo install --path .
```

    This will compile and install the `capf` binary in your Cargo bin path, making it available from anywhere in your terminal.

## Usage

The binary is named `capf`. You can run it from your terminal with various options.

### Basic Example

To process the current directory and save the output to `context.txt`:

```shell script
capf -o context.txt
```

### Advanced Example

To process a specific directory (`./src`), include only Rust (`.rs`) and Toml (`.toml`) files, ignore the `.git` and `target` directories, include file names in the output, and use 8 threads:

```shell script
capf -i ./src --whitelist ".rs,.toml" --ignore-dir ".git,target" -f -t 8 -o rust_project_context.md
```

### Command-line Arguments

Here is a complete list of the available command-line arguments:

| Flag                         | Argument        | Description                                                                                        | Default    |
|------------------------------|-----------------|----------------------------------------------------------------------------------------------------|------------|
| `-i`, `--input`              | `<INPUT>`       | The input directory to process.                                                                    | `./`       |
| `-o`, `--output`             | `<OUTPUT>`      | The output file for the results.                                                                   | (Required) |
| `--whitelist`                | `<EXTENSIONS>`  | A comma-separated list of file extensions or regex patterns to include (e.g., `.rs,.md`).          | (None)     |
| `--blacklist`                | `<EXTENSIONS>`  | A comma-separated list of file extensions or regex patterns to exclude (e.g., `.log,.tmp`).        | (None)     |
| `--ignore-dir`               | `<DIRECTORIES>` | A comma-separated list of directory names or regex patterns to ignore (e.g., `node_modules,.git`). | (None)     |
| `--ignore-file`              | `<FILES>`       | A comma-separated list of file names or regex patterns to ignore (e.g., `*.test.rs,temp.txt`).     | (None)     |
| `-t`, `--threads`            | `<THREADS>`     | The number of threads to use for file processing.                                                  | `4`        |
| `-f`, `--include-file-names` |                 | If set, includes the file path in the output for each file's content.                              | `false`    |
| `-v`, `--verbose`            |                 | Enables verbose logging for debugging.                                                             | `false`    |
| `-h`, `--help`               |                 | Displays the help message.                                                                         |            |
| `-V`, `--version`            |                 | Displays the version of the tool.                                                                  |            |

## Building from Source

If you prefer to build the project without installing it, you can use the standard Cargo build command.

1. **Clone the repository:**

```shell script
git clone <your-repository-url>
    cd contextualize-all-project-files
```

2. **Build the project:**

```shell script
cargo build --release
```

    The compiled binary will be available at `target/release/capf`.

## Contributing

Contributions are welcome! Please feel free to open an issue or submit a pull request if you have any ideas for improvements or new features.