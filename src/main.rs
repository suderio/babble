use clap::{Arg, ArgAction, Command};
use figment::{Figment, providers::{Env, Format, Toml}};
use std::{fs, io, path::Path, collections::HashMap};
use std::io::{Write, BufRead};

fn get_language_extension(language: &str) -> (String, bool) {
    let mut lang_extensions = HashMap::new();
    lang_extensions.insert("rust", "rs");
    lang_extensions.insert("python", "py");
    lang_extensions.insert("javascript", "js");
    lang_extensions.insert("typescript", "ts");
    lang_extensions.insert("java", "java");
    lang_extensions.insert("c", "c");
    lang_extensions.insert("cpp", "cpp");
    lang_extensions.insert("csharp", "cs");
    lang_extensions.insert("go", "go");
    lang_extensions.insert("ruby", "rb");
    lang_extensions.insert("php", "php");
    lang_extensions.insert("swift", "swift");
    lang_extensions.insert("kotlin", "kt");
    lang_extensions.insert("html", "html");
    lang_extensions.insert("css", "css");
    lang_extensions.insert("json", "json");
    lang_extensions.insert("yaml", "yaml");
    lang_extensions.insert("xml", "xml");
    lang_extensions.insert("shell", "sh");
    lang_extensions.insert("bash", "sh");
    lang_extensions.insert("makefile", "makefile");
    lang_extensions.insert("markdown", "md");
    lang_extensions.insert("perl", "pl");
    lang_extensions.insert("r", "r");
    lang_extensions.insert("scala", "scala");
    lang_extensions.insert("haskell", "hs");
    lang_extensions.insert("elixir", "ex");
    lang_extensions.insert("erlang", "erl");
    lang_extensions.insert("dart", "dart");
    lang_extensions.insert("lua", "lua");

    if let Some(ext) = lang_extensions.get(language) {
        (ext.to_string(), false)
    } else {
        (language.to_string(), true)
    }
}

pub fn process_markdown(file_path: &str, output_dir: &str, dry_run: bool, verbose: bool) -> io::Result<()> {
    let content = match fs::read_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", file_path, e);
            return Err(e);
        }
    };

    if !Path::new(output_dir).exists() {
        println!("Output directory '{}' does not exist. Do you want to create it? [y/n]", output_dir);
        let stdin = io::stdin();
        let mut response = String::new();
        stdin.lock().read_line(&mut response)?;
        if response.trim().eq_ignore_ascii_case("y") {
            fs::create_dir_all(output_dir)?;
            println!("Directory '{}' created.", output_dir);
        } else {
            println!("Operation aborted.");
            return Ok(());
        }
    }

    let mut code_blocks = vec![];
    let mut in_code_block = false;
    let mut language = String::new();
    let mut code_buffer = String::new();
    let mut current_title = String::new();

    for line in content.lines() {
        if let Some(level) = line.chars().take_while(|&c| c == '#').count().checked_sub(1) {
            if level > 0 && level <= 6 {
                current_title = line[level + 1..].trim().replace(' ', "_");
            }
        } else if line.starts_with("```") {
            if in_code_block {
                code_blocks.push((current_title.clone(), language.clone(), code_buffer.clone()));
                code_buffer.clear();
                in_code_block = false;
                language.clear();
            } else {
                in_code_block = true;
                language = line[3..].trim().to_string();
                if language.is_empty() {
                    language = "txt".to_string();
                }
            }
        } else if in_code_block {
            code_buffer.push_str(line);
            code_buffer.push('\n');
        }
    }

    for (title, lang, code) in code_blocks {
        let (ext, warn) = get_language_extension(&lang);
        if warn && verbose {
            eprintln!("Warning: Language '{}' not found in known extensions, using '{}' as extension.", lang, ext);
        }

        let file_name = format!("{}.{}", title, ext);
        let file_path = Path::new(output_dir).join(file_name);

        if dry_run {
            if verbose {
                println!("[Dry Run] Would write to: {:?}", file_path);
            }
            continue;
        }

        let mut file = if file_path.exists() {
            fs::OpenOptions::new().append(true).open(&file_path)?
        } else {
            fs::File::create(&file_path)?
        };

        file.write_all(code.as_bytes())?;
        if verbose {
            println!("Wrote to: {:?}", file_path);
        }
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let matches = Command::new("Markdown Processor")
        .version("1.0")
        .author("Your Name <your.email@example.com>")
        .about("Processes Markdown files and extracts code blocks into separate files")
        .arg(
            Arg::new("file")
                .help("The Markdown file to process")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Increases the log level")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("dry-run")
                .short('d')
                .long("dry-run")
                .help("Executes but does not generate files")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("output-dir")
                .short('o')
                .long("output-dir")
                .help("Directory where the files will be generated")
                .default_value("./output"),
        )
        .get_matches();

    let file = matches.get_one::<String>("file").expect("File argument is required");
    let verbose = *matches.get_one::<bool>("verbose").unwrap_or(&false);
    let dry_run = *matches.get_one::<bool>("dry-run").unwrap_or(&false);
    let output_dir = matches.get_one::<String>("output-dir").unwrap_or(&"./output".to_string());

    if verbose {
        println!("Processing file: {}", file);
        println!("Output directory: {}", output_dir);
        println!("Dry run: {}", dry_run);
    }

    process_markdown(file, output_dir, dry_run, verbose)
}

