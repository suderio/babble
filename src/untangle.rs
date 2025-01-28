use log::{info, warn};
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

/// Convert a source code file into a Markdown file with a code block.
pub fn untangle_file(input_file: &Path, output_dir: &Path, dry_run: bool) -> io::Result<()> {
    // Read the input file
    let mut file_content = String::new();
    let extension = input_file
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    fs::File::open(input_file)?.read_to_string(&mut file_content)?;

    // Determine the language from the file extension
    let language = extension_to_language(extension);

    // Create the output file path
    let relative_path = input_file.strip_prefix(".").unwrap_or(input_file);
    let markdown_path = output_dir.join(relative_path).with_extension("md");

    if dry_run {
        info!(
            "Dry run: Would create Markdown file: {:?} with language: {}",
            markdown_path, language
        );
        return Ok(());
    }

    // Ensure the output directory exists
    if let Some(parent) = markdown_path.parent() {
        fs::create_dir_all(parent)?;
    }

    // Write the Markdown content
    let mut output_file = fs::File::create(&markdown_path)?; // Borrow `markdown_path` here
    writeln!(output_file, "```{}", language)?; // Start the code block
    output_file.write_all(file_content.as_bytes())?; // Write the file content
    writeln!(output_file, "```")?; // End the code block

    info!("Created Markdown file: {:?}", markdown_path);

    Ok(())
}

/// Map file extensions to programming languages
fn extension_to_language(extension: &str) -> &str {
    match extension {
        "rs" => "rust",
        "py" => "python",
        "js" => "javascript",
        "ts" => "typescript",
        "java" => "java",
        "c" => "c",
        "cpp" => "cpp",
        "cs" => "csharp",
        "go" => "go",
        "rb" => "ruby",
        "php" => "php",
        "html" => "html",
        "css" => "css",
        "swift" => "swift",
        "kt" => "kotlin",
        "sh" => "bash",
        "lua" => "lua",
        "yaml" | "yml" => "yaml",
        "json" => "json",
        "toml" => "toml",
        "sql" => "sql",
        "xml" => "xml",
        "pl" => "perl",
        "hs" => "haskell",
        "ex" => "elixir",
        "erl" => "erlang",
        "dart" => "dart",
        _ => "",
    }
}
