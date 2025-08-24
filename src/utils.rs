use std::time::Duration;
use serde_json::Value;

/// Converts a string with comma-separated field names into a vector of field paths
pub fn parse_field_selector(selector: &str) -> Vec<Vec<String>> {
    selector
        .split(',')
        .map(|field| field.trim().split('.').map(String::from).collect())
        .collect()
}

/// Creates a timeout duration from milliseconds
pub fn timeout_duration(timeout_ms: u64) -> Duration {
    Duration::from_millis(timeout_ms)
}

/// Filters a JSON value to include only specified fields
pub fn filter_json_fields(value: Value, fields: Vec<Vec<String>>) -> Value {
    // Start with an empty result
    let mut result = Value::Object(serde_json::Map::new());
    
    if let Value::Object(obj) = value {
        for field_path in fields {
            if field_path.is_empty() {
                continue;
            }
            
            // Get the value at the specified path
            let value_at_path = get_value_at_path(&obj, &field_path);
            
            // If we found a value, insert it into the result
            if let Some(val) = value_at_path {
                insert_at_path(&mut result, &field_path, val);
            }
        }
    }
    
    result
}

/// Helper function to get a value at a specific path in a JSON object
fn get_value_at_path(obj: &serde_json::Map<String, Value>, path: &[String]) -> Option<Value> {
    let mut current = obj;
    let last_idx = path.len() - 1;
    
    // Navigate to the parent of the final key
    for (i, key) in path.iter().enumerate() {
        if i == last_idx {
            // Last key, return the value if it exists
            return current.get(key).cloned();
        } else {
            // Not the last key, navigate deeper if possible
            match current.get(key) {
                Some(Value::Object(next_obj)) => current = next_obj,
                _ => return None, // Path doesn't exist
            }
        }
    }
    
    None
}

/// Helper function to insert a value at a specific path in a JSON object
fn insert_at_path(root: &mut Value, path: &[String], value: Value) {
    let mut current = root;
    
    // Navigate to the parent of the final key
    for i in 0..path.len() - 1 {
        let key = &path[i];
        
        // Ensure the current path exists and is an object
        if !current.is_object() {
            *current = Value::Object(serde_json::Map::new());
        }
        
        // Get or create the next level
        let map = current.as_object_mut().unwrap();
        if !map.contains_key(key) {
            map.insert(key.clone(), Value::Object(serde_json::Map::new()));
        }
        
        current = map.get_mut(key).unwrap();
    }
    
    // Insert the value at the final key
    if let Some(last_key) = path.last() {
        if !current.is_object() {
            *current = Value::Object(serde_json::Map::new());
        }
        
        current.as_object_mut().unwrap().insert(last_key.clone(), value);
    }
}