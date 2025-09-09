//! # Tomler
//!
//! A simple lightweight TOML get/set tool library.
//!
//! This library provides functionality to read, modify, and write TOML files
//! with a simple key-based access pattern.

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

/// Represents a TOML document that can be read from and written to files.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TomlDocument {
    data: toml::Value,
}

impl TomlDocument {
    /// Create a new empty TOML document.
    pub fn new() -> Self {
        Self {
            data: toml::Value::Table(toml::map::Map::new()),
        }
    }

    /// Load a TOML document from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read file: {}", path.as_ref().display()))?;

        let data: toml::Value = toml::from_str(&content).with_context(|| {
            format!(
                "Failed to parse TOML from file: {}",
                path.as_ref().display()
            )
        })?;

        Ok(Self { data })
    }

    /// Save the TOML document to a file.
    pub fn to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = toml::to_string_pretty(&self.data).context("Failed to serialize TOML")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write file: {}", path.as_ref().display()))?;

        Ok(())
    }

    /// Get a value from the TOML document using a dot-separated key path.
    pub fn get(&self, key: &str) -> Option<&toml::Value> {
        let keys: Vec<&str> = key.split('.').collect();
        let mut current = &self.data;

        for k in keys {
            match current {
                toml::Value::Table(table) => {
                    current = table.get(k)?;
                }
                _ => return None,
            }
        }

        Some(current)
    }

    /// Set a value in the TOML document using a dot-separated key path.
    pub fn set(&mut self, key: &str, value: toml::Value) -> Result<()> {
        let keys: Vec<&str> = key.split('.').collect();
        if keys.is_empty() {
            anyhow::bail!("Key cannot be empty");
        }

        let mut current = &mut self.data;

        // Navigate to the parent of the target key
        for k in &keys[..keys.len() - 1] {
            match current {
                toml::Value::Table(table) => {
                    current = table
                        .entry(k.to_string())
                        .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
                }
                _ => {
                    anyhow::bail!("Cannot set value: path contains non-table value");
                }
            }
        }

        // Set the final value
        match current {
            toml::Value::Table(table) => {
                table.insert(keys.last().unwrap().to_string(), value);
                Ok(())
            }
            _ => {
                anyhow::bail!("Cannot set value: parent is not a table");
            }
        }
    }

    /// Remove a value from the TOML document using a dot-separated key path.
    pub fn remove(&mut self, key: &str) -> Result<Option<toml::Value>> {
        let keys: Vec<&str> = key.split('.').collect();
        if keys.is_empty() {
            anyhow::bail!("Key cannot be empty");
        }

        let mut current = &mut self.data;

        // Navigate to the parent of the target key
        for k in &keys[..keys.len() - 1] {
            match current {
                toml::Value::Table(table) => {
                    current = table
                        .get_mut(&k.to_string())
                        .ok_or_else(|| anyhow::anyhow!("Key not found: {}", key))?;
                }
                _ => {
                    anyhow::bail!("Cannot remove value: path contains non-table value");
                }
            }
        }

        // Remove the final value
        match current {
            toml::Value::Table(table) => Ok(table.remove(&keys.last().unwrap().to_string())),
            _ => {
                anyhow::bail!("Cannot remove value: parent is not a table");
            }
        }
    }

    /// Get all keys in the TOML document (top-level only).
    pub fn keys(&self) -> Vec<String> {
        match &self.data {
            toml::Value::Table(table) => table.keys().cloned().collect(),
            _ => vec![],
        }
    }

    /// Check if a key exists in the TOML document.
    pub fn contains_key(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

impl Default for TomlDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// Parse a string value into a TOML value, attempting to infer the type.
pub fn parse_value(value: &str) -> toml::Value {
    // Try to parse as different types
    if let Ok(b) = value.parse::<bool>() {
        return toml::Value::Boolean(b);
    }

    if let Ok(i) = value.parse::<i64>() {
        return toml::Value::Integer(i);
    }

    if let Ok(f) = value.parse::<f64>() {
        return toml::Value::Float(f);
    }

    // Try to parse as array
    if value.starts_with('[') && value.ends_with(']') {
        if let Ok(array) = toml::from_str::<toml::Value>(value) {
            if let toml::Value::Array(_) = array {
                return array;
            }
        }
    }

    // Default to string
    toml::Value::String(value.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_new_document() {
        let doc = TomlDocument::new();
        assert_eq!(doc.keys().len(), 0);
    }

    #[test]
    fn test_set_and_get() {
        let mut doc = TomlDocument::new();

        doc.set("name", toml::Value::String("test".to_string()))
            .unwrap();
        assert_eq!(
            doc.get("name"),
            Some(&toml::Value::String("test".to_string()))
        );

        doc.set(
            "database.host",
            toml::Value::String("localhost".to_string()),
        )
        .unwrap();
        assert_eq!(
            doc.get("database.host"),
            Some(&toml::Value::String("localhost".to_string()))
        );
    }

    #[test]
    fn test_remove() {
        let mut doc = TomlDocument::new();

        doc.set("name", toml::Value::String("test".to_string()))
            .unwrap();
        assert!(doc.contains_key("name"));

        let removed = doc.remove("name").unwrap();
        assert_eq!(removed, Some(toml::Value::String("test".to_string())));
        assert!(!doc.contains_key("name"));
    }

    #[test]
    fn test_parse_value() {
        assert_eq!(parse_value("true"), toml::Value::Boolean(true));
        assert_eq!(parse_value("42"), toml::Value::Integer(42));
        assert_eq!(parse_value("3.14"), toml::Value::Float(3.14));
        assert_eq!(
            parse_value("hello"),
            toml::Value::String("hello".to_string())
        );
    }

    #[test]
    fn test_file_operations() {
        let file = NamedTempFile::new().unwrap();
        let path = file.path();

        let mut doc = TomlDocument::new();
        doc.set("name", toml::Value::String("test".to_string()))
            .unwrap();
        doc.set("version", toml::Value::String("1.0".to_string()))
            .unwrap();

        doc.to_file(path).unwrap();

        let loaded_doc = TomlDocument::from_file(path).unwrap();
        assert_eq!(
            loaded_doc.get("name"),
            Some(&toml::Value::String("test".to_string()))
        );
        assert_eq!(
            loaded_doc.get("version"),
            Some(&toml::Value::String("1.0".to_string()))
        );
    }
}
