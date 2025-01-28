use clap::Parser;
use std::path::PathBuf;

/// Configuration for the Markdown code extractor.
#[derive(Parser, Debug)]
#[command(version, author, about)]
pub struct Config {
    /// Input files (supports glob patterns, e.g., src/*.rs or src/*.md)
    #[arg(short, long)]
    pub input_glob: String,

    /// Output directory
    #[arg(short, long, default_value = "./output")]
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
