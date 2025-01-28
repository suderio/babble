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
