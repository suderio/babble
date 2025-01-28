mod config;
mod parser;
mod writer;
mod untangle;

use crate::config::Config;
use glob::glob;
use log::{info, warn};
use std::process;

fn main() {
    // Initialize logging
    env_logger::init();

    let config = Config::from_args_or_exit();

    if config.verbose {
        info!("Configuration: {:?}", config);
    }

    if config.untangle {
        // Untangle mode: Convert source code files to Markdown
        match glob(&config.input_glob) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            if config.verbose {
                                info!("Processing file: {:?}", path);
                            }
                            if let Err(e) = untangle::untangle_file(&path, &config.output_dir, config.dry_run) {
                                warn!("Error processing file {:?}: {}", path, e);
                            }
                        }
                        Err(e) => warn!("Invalid path: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse input glob: {}", e);
                process::exit(1);
            }
        }
    } else {
        // Default behavior: Extract code blocks from Markdown
        match glob(&config.input_glob) {
            Ok(paths) => {
                for entry in paths {
                    match entry {
                        Ok(path) => {
                            if config.verbose {
                                info!("Processing file: {:?}", path);
                            }
                            match parser::extract_code_blocks(&path) {
                                Ok(blocks) => {
                                    if blocks.is_empty() {
                                        warn!("No code blocks found in file: {:?}", path);
                                    } else {
                                        info!("Extracted {} code blocks from {:?}", blocks.len(), path);
                                        if let Err(e) = writer::write_code_blocks(&blocks, &config, &path) {
                                            warn!("Error writing blocks for {:?}: {}", path, e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    warn!("Failed to process {:?}: {}", path, e);
                                }
                            }
                        }
                        Err(e) => warn!("Invalid path: {}", e),
                    }
                }
            }
            Err(e) => {
                eprintln!("Failed to parse input glob: {}", e);
                process::exit(1);
            }
        }
    }
}
