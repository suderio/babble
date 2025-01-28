use std::fs;
use std::path::Path;
use std::process::Command;

fn setup_fixture_dir() -> String {
    let output_dir = "tests/output";
    if Path::new(output_dir).exists() {
        fs::remove_dir_all(output_dir).expect("Failed to clean up output directory");
    }
    fs::create_dir_all(output_dir).expect("Failed to create output directory");
    output_dir.to_string()
}

/// Test default behavior: Extract code blocks from Markdown
#[test]
fn test_extract_code_blocks() {
    let output_dir = setup_fixture_dir();
    let result = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input-glob")
        .arg("tests/fixtures/example.md")
        .arg("--output-dir")
        .arg(&output_dir)
        .output()
        .expect("Failed to run the program");

    assert!(result.status.success(), "Program did not run successfully");

    // Check that the output directory contains the expected files
    let rust_file = Path::new(&output_dir).join("rust/example.rs");
    assert!(
        rust_file.exists(),
        "Expected Rust file {:?} was not created",
        rust_file
    );

    let python_file = Path::new(&output_dir).join("python/example.py");
    assert!(
        python_file.exists(),
        "Expected Python file {:?} was not created",
        python_file
    );
}

/// Test `--tangled` mode: Only process blocks with `:tangle`
#[test]
fn test_tangled_mode() {
    let output_dir = setup_fixture_dir();
    let result = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input-glob")
        .arg("tests/fixtures/example.md")
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--tangled")
        .output()
        .expect("Failed to run the program");

    assert!(result.status.success(), "Program did not run successfully");

    // Check that only the `:tangle` file was created
    let tangled_file = Path::new(&output_dir).join("xpto/something.rs");
    assert!(
        tangled_file.exists(),
        "Expected tangled file {:?} was not created",
        tangled_file
    );

    // Ensure other files (non-`:tangle`) were not created
    let rust_file = Path::new(&output_dir).join("rust/example.rs");
    assert!(
        !rust_file.exists(),
        "Unexpected Rust file {:?} was created in `--tangled` mode",
        rust_file
    );
}

/// Test `--untangle` mode: Convert source files to Markdown
#[test]
fn test_untangle_mode() {
    let output_dir = setup_fixture_dir();
    let result = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input-glob")
        .arg("tests/fixtures/example.rs")
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--untangle")
        .output()
        .expect("Failed to run the program");

    assert!(result.status.success(), "Program did not run successfully");

    // Check that the Markdown file was created
    let markdown_file = Path::new(&output_dir).join("tests/fixtures/example.md");
    assert!(
        markdown_file.exists(),
        "Expected Markdown file {:?} was not created",
        markdown_file
    );

    // Verify the content of the Markdown file
    let content = fs::read_to_string(markdown_file).expect("Failed to read Markdown file");
    assert!(
        content.contains("```rust"),
        "Markdown file does not contain the expected Rust code block"
    );
}

/// Test `--dry-run` mode
#[test]
fn test_dry_run_mode() {
    let output_dir = setup_fixture_dir();
    let result = Command::new("cargo")
        .arg("run")
        .arg("--")
        .arg("--input-glob")
        .arg("tests/fixtures/example.md")
        .arg("--output-dir")
        .arg(&output_dir)
        .arg("--dry-run")
        .output()
        .expect("Failed to run the program");

    assert!(result.status.success(), "Program did not run successfully");

    // Ensure no files were created
    let rust_file = Path::new(&output_dir).join("rust/example.rs");
    assert!(
        !rust_file.exists(),
        "Rust file {:?} was unexpectedly created in dry-run mode",
        rust_file
    );
}
