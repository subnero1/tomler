use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tomler::{parse_value, TomlDocument};

#[derive(Parser)]
#[command(
    name = "tomler",
    about = "A simple lightweight TOML get/set tool",
    version = env!("CARGO_PKG_VERSION"),
    author = "subnero1"
)]
struct Cli {
    /// TOML file to operate on
    #[arg(short, long, default_value = "config.toml")]
    file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Get a value from the TOML file
    Get {
        /// Key to get (supports dot notation like 'database.host')
        key: String,
    },
    /// Set a value in the TOML file
    Set {
        /// Key to set (supports dot notation like 'database.host')
        key: String,
        /// Value to set (auto-detects type: string, number, boolean, array)
        value: String,
    },
    /// Remove a key from the TOML file
    Remove {
        /// Key to remove (supports dot notation like 'database.host')
        key: String,
    },
    /// List all keys in the TOML file
    Keys,
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Get { key } => {
            let doc = TomlDocument::from_file(&cli.file)
                .with_context(|| format!("Failed to load TOML file: {}", cli.file.display()))?;

            match doc.get(&key) {
                Some(value) => {
                    println!("{}", format_value(value));
                }
                None => {
                    eprintln!("Key '{}' not found", key);
                    std::process::exit(1);
                }
            }
        }
        Commands::Set { key, value } => {
            let mut doc = if cli.file.exists() {
                TomlDocument::from_file(&cli.file)
                    .with_context(|| format!("Failed to load TOML file: {}", cli.file.display()))?
            } else {
                TomlDocument::new()
            };

            let parsed_value = parse_value(&value);
            doc.set(&key, parsed_value)
                .with_context(|| format!("Failed to set key '{}'", key))?;

            doc.to_file(&cli.file)
                .with_context(|| format!("Failed to save TOML file: {}", cli.file.display()))?;

            println!("Set '{}' = '{}'", key, value);
        }
        Commands::Remove { key } => {
            let mut doc = TomlDocument::from_file(&cli.file)
                .with_context(|| format!("Failed to load TOML file: {}", cli.file.display()))?;

            match doc.remove(&key)? {
                Some(removed_value) => {
                    doc.to_file(&cli.file).with_context(|| {
                        format!("Failed to save TOML file: {}", cli.file.display())
                    })?;
                    println!("Removed '{}' (was: {})", key, format_value(&removed_value));
                }
                None => {
                    eprintln!("Key '{}' not found", key);
                    std::process::exit(1);
                }
            }
        }
        Commands::Keys => {
            let doc = TomlDocument::from_file(&cli.file)
                .with_context(|| format!("Failed to load TOML file: {}", cli.file.display()))?;

            let keys = doc.keys();
            if keys.is_empty() {
                println!("No keys found");
            } else {
                for key in keys {
                    println!("{}", key);
                }
            }
        }
    }

    Ok(())
}

fn format_value(value: &toml::Value) -> String {
    match value {
        toml::Value::String(s) => s.clone(),
        toml::Value::Integer(i) => i.to_string(),
        toml::Value::Float(f) => f.to_string(),
        toml::Value::Boolean(b) => b.to_string(),
        toml::Value::Array(arr) => {
            let items: Vec<String> = arr.iter().map(format_value).collect();
            format!("[{}]", items.join(", "))
        }
        toml::Value::Table(_) => "[table]".to_string(),
        toml::Value::Datetime(dt) => dt.to_string(),
    }
}
