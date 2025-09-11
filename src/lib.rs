//! Library functions for tomler. Keep logic here so tests can exercise it.

use toml_edit::{DocumentMut, Item, Table, Value};

/// Infer a toml_edit::Value from a string input.
/// Supports:
/// - booleans ("true"/"false")
/// - integer (i64)
/// - float (f64)
/// - simple arrays (comma separated, no quotes with commas: "1,2,3" or "a,b,c")
/// - everything else -> string
pub fn infer_value(s: &str) -> Value {
    let s_trim = s.trim();

    // Simple array detection: comma-separated values without complex quote handling
    if s_trim.contains(',') && !is_quoted_string(s_trim) {
        let parts: Vec<&str> = s_trim.split(',').map(|p| p.trim()).collect();
        if parts.len() > 1 {
            let vals: Vec<Value> = parts.iter().map(|p| infer_single_value(p)).collect();
            let mut array = toml_edit::Array::new();
            for val in vals {
                array.push(val);
            }
            return Value::Array(array);
        }
    }

    infer_single_value(s_trim)
}

/// Check if a string is quoted (surrounded by matching quotes)
fn is_quoted_string(s: &str) -> bool {
    (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\''))
}

/// Infer a single value (no array detection)
fn infer_single_value(s: &str) -> Value {
    let s_trim = s.trim();

    // Remove quotes if present
    let unquoted = if is_quoted_string(s_trim) {
        &s_trim[1..s_trim.len()-1]
    } else {
        s_trim
    };

    // Type inference - using match for cleaner flow
    if unquoted.eq_ignore_ascii_case("true") {
        Value::Boolean(toml_edit::Formatted::new(true))
    } else if unquoted.eq_ignore_ascii_case("false") {
        Value::Boolean(toml_edit::Formatted::new(false))
    } else if let Ok(i) = unquoted.parse::<i64>() {
        Value::Integer(toml_edit::Formatted::new(i))
    } else if let Ok(f) = unquoted.parse::<f64>() {
        Value::Float(toml_edit::Formatted::new(f))
    } else {
        Value::String(toml_edit::Formatted::new(unquoted.to_string().into()))
    }
}

/// Get a textual representation of a key from a Document.
/// Returns `Some` if key exists, else None.
/// This prints raw TOML token for the value (so strings include quotes).
pub fn get_value(doc: &DocumentMut, key: &str) -> Option<String> {
    // Handle nested keys with dot notation
    let parts: Vec<&str> = key.split('.').collect();
    let mut current = doc.as_item();

    for part in parts {
        current = current.get(part)?;
    }

    Some(current.to_string().trim().to_string())
}

/// Set a nested (dot-notated) key in the Document, creating tables as necessary.
/// Overwrites existing value at that key.
pub fn set_nested_in_document(doc: &mut DocumentMut, key: &str, v: Value) {
    let parts: Vec<&str> = key.split('.').collect();
    assert!(!parts.is_empty(), "key must be non-empty");

    // Walk/create tables
    let last = parts.last().unwrap();
    let mut table: &mut Table = doc.as_table_mut();

    for part in &parts[..parts.len().saturating_sub(1)] {
        // entry(part) returns an Entry. If missing, insert a Table.
        // If there's a non-table item present at this key, replace it with a table.
        let ent = table.entry(part);
        match ent {
            toml_edit::Entry::Vacant(vacant) => {
                vacant.insert(Item::Table(Table::new()));
            }
            toml_edit::Entry::Occupied(mut occupied) => {
                if !occupied.get().is_table() {
                    occupied.insert(Item::Table(Table::new()));
                }
            }
        }
        table = table[part].as_table_mut().expect("table created above");
    }

    table[*last] = Item::Value(v);
}

#[cfg(test)]
mod tests {
    use super::*;
    use toml_edit::DocumentMut;

    #[test]
    fn infer_primitives() {
        assert!(infer_value("true").as_bool().unwrap());
        assert_eq!(infer_value("False").as_bool().unwrap(), false);
        assert_eq!(infer_value("  42 ").as_integer().unwrap(), 42);
        assert!((infer_value("3.14").as_float().unwrap() - 3.14).abs() < 1e-10);
        // quoted is string
        assert_eq!(infer_value("\"hello\"").as_str().unwrap(), "hello");
        // unquoted words are strings
        assert_eq!(infer_value("hello").as_str().unwrap(), "hello");
    }

    #[test]
    fn infer_arrays() {
        // Simple comma-separated integers
        let v = infer_value("1,2,3");
        let arr = v.as_array().expect("should be array");
        assert_eq!(arr.len(), 3);
        let first = arr.iter().next().unwrap();
        assert_eq!(first.as_integer().unwrap(), 1);

        // Simple comma-separated booleans
        let v2 = infer_value("true,false,true");
        let a2 = v2.as_array().unwrap();
        assert_eq!(a2.len(), 3);
        let items: Vec<&Value> = a2.iter().collect();
        match &items[1] {
            Value::Boolean(formatted_bool) => {
                assert_eq!(*formatted_bool.value(), false);
            }
            _ => panic!("Expected boolean value"),
        }

        // Simple comma-separated strings (no commas within quotes)
        let v3 = infer_value("apple,banana,cherry");
        let a3 = v3.as_array().unwrap();
        let items: Vec<&Value> = a3.iter().collect();
        assert_eq!(items.len(), 3);
        match &items[0] {
            Value::String(formatted_str) => {
                assert_eq!(formatted_str.value().as_str(), "apple");
            }
            _ => panic!("Expected string value"),
        }

        // Quoted string should not be treated as array
        let v4 = infer_value("\"a,b,c\"");
        assert!(v4.as_str().is_some());
        assert_eq!(v4.as_str().unwrap(), "a,b,c");
    }

    #[test]
    fn set_nested_creates_tables_and_sets_value() {
        let src = r#"# comment
[server]
host = "localhost"
"#;
        let mut doc: DocumentMut = src.parse().unwrap();
        set_nested_in_document(&mut doc, "server.port", Value::Integer(toml_edit::Formatted::new(8000)));
        // check - use our getter function which handles nested keys
        assert!(get_value(&doc, "server.port").is_some());
        assert_eq!(get_value(&doc, "server.port").unwrap(), "8000");

        // create nested deeper - simple array
        set_nested_in_document(&mut doc, "servers.main.ports", infer_value("80,443"));
        assert!(get_value(&doc, "servers.main.ports").is_some());
        assert_eq!(get_value(&doc, "servers.main.ports").unwrap(), "[80, 443]");
    }

    #[test]
    fn simple_array_parsing() {
        // Test the simplified array parsing
        assert_eq!(infer_value("a,b,c").as_array().unwrap().len(), 3);
        assert_eq!(infer_value("1,2,3").as_array().unwrap().len(), 3);

        // Quoted strings should not be split
        assert!(infer_value("\"a,b,c\"").as_str().is_some());
        assert!(infer_value("'x,y,z'").as_str().is_some());

        // Single values should not be arrays
        assert!(infer_value("single").as_str().is_some());
        assert!(infer_value("42").as_integer().is_some());
    }
}
