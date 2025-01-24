# md-babble

**md-babble** is a command-line tool inspired by the functionality of Emacs' `org-babel`. It processes Markdown files, extracts code blocks, and generates source files for each block based on its specified language.

## Features

- Extracts code blocks from Markdown files and saves them as individual files.
- Supports multiple languages with proper file extensions (e.g., `.rs` for Rust, `.py` for Python).
- Handles nested headers (`#`, `##`, ... `######`) to name files appropriately.
- Allows appending multiple code blocks to the same file if they belong to the same title and language.
- Provides command-line options for:
  - Verbose logging.
  - Dry-run execution (simulate without generating files).
  - Custom output directory.
- Gracefully handles missing output directories by prompting the user to create them.

## Installation

You can install **md-babble** using [Cargo](https://doc.rust-lang.org/cargo/):

```bash
cargo install md-babble
```

Alternatively, clone this repository and build it locally:

```bash
git clone https://github.com/your-username/md-babble.git
cd md-babble
cargo build --release
```

The executable will be available in `./target/release/md-babble`.

## Usage

To process a Markdown file:

```bash
md-babble <file> [OPTIONS]
```

### Options

- `-v`, `--verbose`  
  Increase logging verbosity.

- `-d`, `--dry-run`  
  Simulate execution without generating files.

- `-o`, `--output-dir <DIR>`  
  Specify the output directory for generated files (default: `./output`).

### Example

Given a Markdown file `example.md`:

```markdown
# MyProject

## Main
```rust
fn main() {
    println!("Hello, md-babble!");
}
    ```

```python
print("Hello, md-babble!")
    ```
```

Run the following command:

```bash
md-babble example.md -v -o ./generated
```

Generated files:

- `generated/Main.rs`
- `generated/Main.py`

## Supported Languages

md-babble supports 30+ languages, including:

- Rust (`.rs`)
- Python (`.py`)
- JavaScript (`.js`)
- TypeScript (`.ts`)
- Java (`.java`)
- C (`.c`), C++ (`.cpp`)
- Go (`.go`)
- Ruby (`.rb`)
- PHP (`.php`)
- Swift (`.swift`)
- Kotlin (`.kt`)
- Shell (`.sh`)

If a language is not recognized, md-babble will use the language name as the file extension and issue a warning.

## Future Features

- **Org-mode support:** Extend functionality to process `.org` files directly.
- **Code block execution:** Evaluate code blocks in supported languages, similar to `org-babel`.
- **Configuration files:** Allow users to define custom file extensions and behaviors.
- **Inline error handling:** More informative error messages with suggestions.

## Contributing

Contributions are welcome! Here’s how you can help:

1. Fork the repository.
2. Create a feature branch (`git checkout -b feature/your-feature`).
3. Commit your changes (`git commit -m 'Add your feature'`).
4. Push to the branch (`git push origin feature/your-feature`).
5. Open a pull request.

Please ensure your code follows Rust idioms and includes tests where appropriate.

## License

md-babble is licensed under the [MIT License](LICENSE).

## Acknowledgments

- Inspired by Emacs' `org-babel` and the power of literate programming.
- Built with ❤️ in Rust.
