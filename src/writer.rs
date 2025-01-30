use crate::parser::CodeBlock;
use crate::config::Config;
use log::{info, warn};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};

/// Write code blocks to the output directory.
pub fn write_code_blocks(blocks: &[CodeBlock], config: &Config, source_file: &Path) -> io::Result<()> {
    let file_stem = source_file
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown");

    let extensions = config.language_extensions(); // Retrieve language-extension mappings

    for (index, block) in blocks.iter().enumerate() {
        if config.tangled && block.tangle_path.is_none() {
            // Skip blocks without :tangle if --tangled is set
            warn!(
                "Skipping block {}: No :tangle keyword found, and --tangled is enabled.",
                index + 1
            );
            continue;
        }

        if let Some(ref lang) = block.language {
            // Retrieve the file extension for the given language, default to ".txt" if unknown
            let extension = extensions.get(lang).map(String::as_str).unwrap_or("txt");

            // Determine the file path to use
            let output_path: PathBuf = if let Some(tangle_path) = &block.tangle_path {
                config.output_dir.join(tangle_path)
            } else {
                config.output_dir.join(format!("{}.{}", file_stem, extension))
            };

            if config.dry_run {
                info!(
                    "Dry run: Would create file: {:?} with content length: {}",
                    output_path,
                    block.content.len()
                );
            } else {
                if let Some(parent) = output_path.parent() {
                    fs::create_dir_all(parent)?;
                }
                if config.verbose {
                    info!("Creating {:?}", output_path);
                }

                let mut file = File::create(&output_path)?;
                file.write_all(block.content.as_bytes())?;
                info!("Created file: {:?}", output_path);
            }
        } else {
            warn!("Skipping block {}: No language specifier found.", index + 1);
        }
    }

    Ok(())
}

