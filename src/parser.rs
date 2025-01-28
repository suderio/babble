use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use pulldown_cmark::{CodeBlockKind, Event};
use pulldown_cmark::Parser as MDParser;
use std::fs;
use std::io::{self, Read};

/// Represents a code block extracted from Markdown.
#[derive(Debug)]
pub struct CodeBlock {
    /// Language of the code block (e.g., "rust", "python").
    pub language: Option<String>,
    /// Content of the code block.
    pub content: String,
    /// Optional target file path specified by `:tangle`.
    pub tangle_path: Option<String>,
}

/// Extract code blocks from a Markdown file.
///
/// This function parses a Markdown file, identifies fenced code blocks,
/// and extracts their language, content, and optional `:tangle` path.
///
/// # Arguments
/// - `file_path`: Path to the Markdown file to process.
///
/// # Returns
/// A vector of `CodeBlock` structs containing the parsed code blocks.
pub fn extract_code_blocks(file_path: &std::path::Path) -> io::Result<Vec<CodeBlock>> {
    let mut file_content = String::new();
    fs::File::open(file_path)?.read_to_string(&mut file_content)?;

    let parser = MDParser::new(&file_content);
    let mut blocks = Vec::new();
    let mut current_block = None;

    for event in parser {
        match event {
            Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                let parts: Vec<&str> = info.split_whitespace().collect();
                let language = parts.get(0).map(|s| s.to_string());
                let tangle_path = parts
                    .iter()
                    .find(|&&part| part.starts_with(":tangle"))
                    .and_then(|t| t.strip_prefix(":tangle").map(|s| s.trim().to_string()));

                current_block = Some(CodeBlock {
                    language,
                    content: String::new(),
                    tangle_path,
                });
            }
            Event::Text(text) => {
                if let Some(block) = current_block.as_mut() {
                    block.content.push_str(&text);
                }
            }
            Event::End(pulldown_cmark::Tag::CodeBlock(_)) => {
                if let Some(block) = current_block.take() {
                    blocks.push(block);
                }
            }
            _ => {}
        }
    }

    Ok(blocks)
}

/// Configuration for the Markdown code extractor.
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Config {
    /// Input files (supports glob patterns, e.g., src/*.rs or src/*.md)
    #[arg(short, long)]
    pub input_glob: String,

    /// Output directory
    #[arg(short, long, parse(from_os_str), default_value = "./output")]
    pub output_dir: PathBuf,

    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,

    /// Perform a dry run (no changes to the file system)
    #[arg(long)]
    pub dry_run: bool,

    /// Only process blocks with :tangle
    #[arg(long)]
    pub tangled: bool,

    /// Untangle mode: convert source code files to Markdown
    #[arg(long)]
    pub untangle: bool,

    /// Language-to-extension mapping (e.g., rust=rs). Can be specified multiple times.
    #[arg(long)]
    pub extension: Vec<String>,
}

impl Config {
    /// Returns the language-to-extension mapping, including defaults and user overrides.
    pub fn language_extensions(&self) -> HashMap<String, String> {
        let mut extensions = Self::default_extensions();

        // Override defaults with user-provided mappings
        for pair in &self.extension {
            if let Some((lang, ext)) = pair.split_once('=') {
                extensions.insert(lang.to_string(), ext.to_string());
            }
        }

        extensions
    }

    /// Default language-to-extension mapping for the 40 most used languages.
    fn default_extensions() -> HashMap<String, String> {
        [
            ("rust", "rs"),
            ("python", "py"),
            ("javascript", "js"),
            ("typescript", "ts"),
            ("java", "java"),
            ("c", "c"),
            ("cpp", "cpp"),
            ("csharp", "cs"),
            ("go", "go"),
            ("ruby", "rb"),
            ("php", "php"),
            ("html", "html"),
            ("css", "css"),
            ("swift", "swift"),
            ("kotlin", "kt"),
            ("scala", "scala"),
            ("perl", "pl"),
            ("r", "r"),
            ("dart", "dart"),
            ("shell", "sh"),
            ("bash", "sh"),
            ("lua", "lua"),
            ("yaml", "yaml"),
            ("json", "json"),
            ("toml", "toml"),
            ("xml", "xml"),
            ("sql", "sql"),
            ("markdown", "md"),
            ("makefile", "mk"),
            ("dockerfile", "dockerfile"),
            ("haskell", "hs"),
            ("elixir", "ex"),
            ("erlang", "erl"),
            ("nim", "nim"),
            ("scheme", "scm"),
            ("clojure", "clj"),
            ("lisp", "lisp"),
            ("vb", "vb"),
            ("fsharp", "fs"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
    }
}
