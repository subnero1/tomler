use clap::{Parser, Subcommand};
use std::fs;
use std::process;

use tomler::{get_value, infer_value, set_nested_in_document};

#[derive(Parser)]
#[command(name = "toml-cli")]
#[command(about = "Edit TOML files in-place with simple type inference and nested keys")]
struct Cli {
    /// TOML file path (default: config.toml)
    #[arg(short, long, default_value = "config.toml")]
    file: String,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a value by key (dot notation)
    Get { key: String },

    /// Set a value by key (dot notation)
    Set { key: String, value: String },
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let raw = fs::read_to_string(&cli.file)
        .map_err(|e| anyhow::anyhow!("failed to read {}: {}", cli.file, e))?;

    let mut doc: toml_edit::DocumentMut = raw
        .parse()
        .map_err(|e| anyhow::anyhow!("failed to parse toml file {}: {}", cli.file, e))?;

    match cli.command {
        Commands::Get { key } => {
            match get_value(&doc, &key) {
                Some(s) => {
                    println!("{}", s);
                    return Ok(());
                }
                None => {
                    eprintln!("Key not found: {}", key);
                    process::exit(2);
                }
            }
        }
        Commands::Set { key, value } => {
            let v = infer_value(&value);
            set_nested_in_document(&mut doc, &key, v);
            fs::write(&cli.file, doc.to_string())
                .map_err(|e| anyhow::anyhow!("failed to write {}: {}", cli.file, e))?;
        }
    }

    Ok(())
}
