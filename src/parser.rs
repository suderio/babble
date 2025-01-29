use pulldown_cmark::{CodeBlockKind, Event, Parser};
use std::fs;
use std::io::{self, Read};
use log::info;

/// Represents a code block extracted from Markdown.
#[derive(Debug)]
pub struct CodeBlock {
    pub language: Option<String>,
    pub content: String,
    pub tangle_path: Option<String>, // Optional target file path specified by :tangle
}

/// Extract code blocks from a Markdown file.
pub fn extract_code_blocks(file_path: &std::path::Path) -> io::Result<Vec<CodeBlock>> {
    let mut file_content = String::new();
    fs::File::open(file_path)?.read_to_string(&mut file_content)?;

    let parser = Parser::new(&file_content);
    let mut blocks = Vec::new();
    let mut current_block = None;

    for event in parser {
        match event {
            Event::Start(pulldown_cmark::Tag::CodeBlock(CodeBlockKind::Fenced(info))) => {
                let parts: Vec<&str> = info.split_whitespace().collect();
                info!("parts {:?}", parts);
                let language = parts.get(0).map(|s| s.to_string());
                let tangle_path = parts
                    .iter()
                    .position(|&s| s == ":tangle")
                    .and_then(|i| Some(parts.get(i + 1)?.to_string()));
                    info!("tangle_path: {:?}", tangle_path);
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
