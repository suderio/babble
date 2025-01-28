# markdown-babble

**markdown-babble** is a command-line tool inspired by the functionality of Emacs' `org-babel`. It processes Markdown files, extracts code blocks, and generates source files for each block based on its specified language.

It is a versatile and efficient command-line tool that allows you to process Markdown and source code files. It provides two primary modes of operation:

1. Tangle Mode: Extracts code blocks from Markdown files and saves them as separate source code files.


2. Untangle Mode: Converts source code files into Markdown files with appropriate fenced code blocks.



This tool is perfect for developers, technical writers, and researchers who need a seamless way to manage Markdown files containing code snippets or document their source code effectively.


---

## Features

### General Features:

Process Markdown files with code blocks in various programming languages.

Supports over 40 programming languages with default file extensions.

Glob support: Specify multiple files or directories using patterns like src/**/*.md.

Customizable file extensions: Override defaults for specific languages using --extension.

Handles :tangle directives to define custom target file paths for code blocks.


### Modes:

1. Tangle Mode (Default):

Extracts code blocks from Markdown files into individual source files.

Supports :tangle <filename> to customize the target filename and location.



2. Untangle Mode (--untangle):

Converts source code files into Markdown files with correctly formatted fenced code blocks.

Maintains directory structure and filenames.




### Additional Features:

Dry-Run Mode (--dry-run): Simulate the process without creating or modifying any files.

Verbose Logging (--verbose): Provides detailed information about the operations being performed.

Selective Processing (--tangled): Only process code blocks with the :tangle directive.


---

## Installation

### Prerequisites

Rust and Cargo installed on your system. Install them via Rustup:

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh


### Build

Clone the repository and build the project:

git clone https://github.com/your-username/markdown-code-extractor.git
cd markdown-code-extractor
cargo build --release

The binary will be available at target/release/markdown-code-extractor.


---

## Usage

Syntax

markdown-babble [FLAGS] --input-glob <GLOB_PATTERN> [OPTIONS]

Command-Line Arguments


---

## Examples

1. Extract Code Blocks (Tangle Mode)

Extract all code blocks from Markdown files in the docs directory:

markdown-babble --input-glob "docs/**/*.md" --output-dir "./output"

2. Process Only Tangled Blocks

Only process blocks with the :tangle directive:

markdown-babble --input-glob "docs/**/*.md" --tangled

3. Convert Source Code to Markdown (Untangle Mode)

Convert Rust source code files into Markdown:

markdown-babble --input-glob "src/**/*.rs" --output-dir "./docs" --untangle

4. Dry Run

Simulate extracting code blocks without creating any files:

markdown-babble --input-glob "docs/**/*.md" --dry-run

5. Customize Extensions

Set a custom extension for Rust files:

markdown-babble --input-glob "docs/**/*.md" --extension rust=rustfile


---

## Markdown File Example

Input (example.md):

```rust :tangle src/hello.rs
fn main() {
    println!("Hello, world!");
    }
```

```python
    print("Hello, Python!")
```

    ### Output Directory (`output/`):
```plaintext
    output/
    ├── rust/
    │   └── example.rs
    ├── python/
    │   └── example.py
    └── src/
        └── hello.rs
```

---

## Development

### Running Tests

The project includes a comprehensive set of integration tests. To run the tests:

cargo test

Example Test Case

Input Markdown:

```rust :tangle src/example.rs
        fn main() {
            println!("Hello, world!");
            }
```

- Expected Output:
```plaintext
            output/
            └── src/
                └── example.rs
```

---

## Supported Languages

The tool supports over 40 languages, including:

                Rust (.rs)

                Python (.py)

                JavaScript (.js)

                TypeScript (.ts)

                Java (.java)

                C (.c)

                C++ (.cpp)

                Go (.go)

                Ruby (.rb)

                PHP (.php)

                HTML (.html)

                CSS (.css)


You can override these defaults with the --extension flag.


---

## Contributing

Contributions are welcome! To contribute:

1. Fork the repository.

2. Create a feature branch.

3. Submit a pull request with your changes.

For bugs or feature requests, please open an issue on GitHub.
    
## Future Features

- **Code block execution:** Evaluate code blocks in supported languages, similar to `org-babel`.
- **Configuration files:** Allow users to define custom file extensions and behaviors.
- **Inline error handling:** More informative error messages with suggestions.

## License

markdown-babble is licensed under the [MIT License](LICENSE).

## Acknowledgments

- Inspired by Emacs' `org-babel` and the power of literate programming.
