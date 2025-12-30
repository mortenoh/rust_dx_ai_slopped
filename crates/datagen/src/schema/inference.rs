//! JSON Schema inference from data samples.
//!
//! Analyzes data samples to infer a JSON Schema that describes the structure.

use serde::Serialize;
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};

/// Infer a JSON Schema from serializable data samples.
///
/// # Arguments
///
/// * `records` - Slice of serializable records to analyze
///
/// # Returns
///
/// A JSON Schema that describes the structure of the data.
///
/// # Example
///
/// ```
/// use dx_datagen::schema::infer_schema;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Person {
///     name: String,
///     age: u32,
/// }
///
/// let records = vec![
///     Person { name: "Alice".into(), age: 30 },
///     Person { name: "Bob".into(), age: 25 },
/// ];
///
/// let schema = infer_schema(&records);
/// assert!(schema.get("properties").is_some());
/// ```
pub fn infer_schema<T: Serialize>(records: &[T]) -> Value {
    let values: Vec<Value> = records
        .iter()
        .filter_map(|r| serde_json::to_value(r).ok())
        .collect();

    infer_schema_from_values(&values)
}

/// Infer a JSON Schema from JSON values.
///
/// # Arguments
///
/// * `values` - Slice of JSON values to analyze
///
/// # Returns
///
/// A JSON Schema that describes the structure of the data.
pub fn infer_schema_from_values(values: &[Value]) -> Value {
    if values.is_empty() {
        return json!({});
    }

    // Collect type information from all values
    let mut inferrer = SchemaInferrer::new();
    for value in values {
        inferrer.observe(value);
    }

    inferrer.to_schema()
}

/// Tracks observed types and properties to infer schema.
struct SchemaInferrer {
    /// Observed types (null, boolean, integer, number, string, array, object)
    types: HashSet<String>,
    /// For objects: property name -> nested inferrer
    properties: HashMap<String, SchemaInferrer>,
    /// Count of observations per property (to determine required)
    property_counts: HashMap<String, usize>,
    /// Total observations
    observation_count: usize,
    /// For arrays: inferrer for items
    items_inferrer: Option<Box<SchemaInferrer>>,
    /// For strings: observed formats
    string_formats: HashSet<String>,
    /// For numbers: min/max observed
    number_min: Option<f64>,
    number_max: Option<f64>,
    /// For integers only
    all_integers: bool,
    /// For strings: min/max length
    string_min_len: Option<usize>,
    string_max_len: Option<usize>,
    /// For arrays: min/max items
    array_min_items: Option<usize>,
    array_max_items: Option<usize>,
    /// Enum values (if small set of strings)
    enum_values: HashSet<String>,
}

impl SchemaInferrer {
    fn new() -> Self {
        Self {
            types: HashSet::new(),
            properties: HashMap::new(),
            property_counts: HashMap::new(),
            observation_count: 0,
            items_inferrer: None,
            string_formats: HashSet::new(),
            number_min: None,
            number_max: None,
            all_integers: true,
            string_min_len: None,
            string_max_len: None,
            array_min_items: None,
            array_max_items: None,
            enum_values: HashSet::new(),
        }
    }

    fn observe(&mut self, value: &Value) {
        self.observation_count += 1;

        match value {
            Value::Null => {
                self.types.insert("null".to_string());
            }
            Value::Bool(_) => {
                self.types.insert("boolean".to_string());
            }
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    self.types.insert("integer".to_string());
                } else {
                    self.types.insert("number".to_string());
                    self.all_integers = false;
                }
                if let Some(f) = n.as_f64() {
                    self.number_min = Some(self.number_min.map(|m| m.min(f)).unwrap_or(f));
                    self.number_max = Some(self.number_max.map(|m| m.max(f)).unwrap_or(f));
                }
            }
            Value::String(s) => {
                self.types.insert("string".to_string());
                let len = s.len();
                self.string_min_len = Some(self.string_min_len.map(|m| m.min(len)).unwrap_or(len));
                self.string_max_len = Some(self.string_max_len.map(|m| m.max(len)).unwrap_or(len));

                // Detect format
                if let Some(fmt) = detect_format(s) {
                    self.string_formats.insert(fmt);
                }

                // Track enum values (limit to avoid huge sets)
                if self.enum_values.len() < 20 {
                    self.enum_values.insert(s.clone());
                }
            }
            Value::Array(arr) => {
                self.types.insert("array".to_string());
                let len = arr.len();
                self.array_min_items =
                    Some(self.array_min_items.map(|m| m.min(len)).unwrap_or(len));
                self.array_max_items =
                    Some(self.array_max_items.map(|m| m.max(len)).unwrap_or(len));

                // Infer items schema
                if self.items_inferrer.is_none() {
                    self.items_inferrer = Some(Box::new(SchemaInferrer::new()));
                }
                if let Some(ref mut inferrer) = self.items_inferrer {
                    for item in arr {
                        inferrer.observe(item);
                    }
                }
            }
            Value::Object(obj) => {
                self.types.insert("object".to_string());
                for (key, val) in obj {
                    *self.property_counts.entry(key.clone()).or_insert(0) += 1;
                    self.properties
                        .entry(key.clone())
                        .or_insert_with(SchemaInferrer::new)
                        .observe(val);
                }
            }
        }
    }

    fn to_schema(&self) -> Value {
        let mut schema = Map::new();

        // Determine type(s)
        let types: Vec<&String> = self.types.iter().collect();

        if types.is_empty() {
            return json!({});
        }

        if types.len() == 1 {
            schema.insert("type".to_string(), json!(types[0]));
        } else {
            // Multiple types
            schema.insert("type".to_string(), json!(types));
        }

        // Add type-specific constraints
        if self.types.contains("string") && self.types.len() == 1 {
            self.add_string_constraints(&mut schema);
        }

        if (self.types.contains("integer") || self.types.contains("number"))
            && self.types.len() == 1
        {
            self.add_number_constraints(&mut schema);
        }

        if self.types.contains("array") && self.types.len() == 1 {
            self.add_array_constraints(&mut schema);
        }

        if self.types.contains("object") && self.types.len() == 1 {
            self.add_object_constraints(&mut schema);
        }

        Value::Object(schema)
    }

    fn add_string_constraints(&self, schema: &mut Map<String, Value>) {
        // Add format if consistent
        if self.string_formats.len() == 1 {
            let fmt = self.string_formats.iter().next().unwrap();
            schema.insert("format".to_string(), json!(fmt));
        }

        // Add length constraints
        if let Some(min) = self.string_min_len {
            if min > 0 {
                schema.insert("minLength".to_string(), json!(min));
            }
        }
        if let Some(max) = self.string_max_len {
            schema.insert("maxLength".to_string(), json!(max));
        }

        // Add enum if small fixed set (and more than 1 observation)
        if self.observation_count > 1
            && self.enum_values.len() <= 10
            && self.enum_values.len() < self.observation_count
        {
            let mut values: Vec<&String> = self.enum_values.iter().collect();
            values.sort();
            schema.insert("enum".to_string(), json!(values));
            schema.remove("type"); // enum implies type
            schema.remove("minLength");
            schema.remove("maxLength");
        }
    }

    fn add_number_constraints(&self, schema: &mut Map<String, Value>) {
        // Use integer type if all values were integers
        let is_integer = self.all_integers && self.types.contains("integer");
        if is_integer {
            schema.insert("type".to_string(), json!("integer"));
        }

        if let Some(min) = self.number_min {
            if is_integer {
                schema.insert("minimum".to_string(), json!(min as i64));
            } else {
                schema.insert("minimum".to_string(), json!(min));
            }
        }
        if let Some(max) = self.number_max {
            if is_integer {
                schema.insert("maximum".to_string(), json!(max as i64));
            } else {
                schema.insert("maximum".to_string(), json!(max));
            }
        }
    }

    fn add_array_constraints(&self, schema: &mut Map<String, Value>) {
        if let Some(min) = self.array_min_items {
            if min > 0 {
                schema.insert("minItems".to_string(), json!(min));
            }
        }
        if let Some(max) = self.array_max_items {
            schema.insert("maxItems".to_string(), json!(max));
        }

        // Add items schema
        if let Some(ref inferrer) = self.items_inferrer {
            schema.insert("items".to_string(), inferrer.to_schema());
        }
    }

    fn add_object_constraints(&self, schema: &mut Map<String, Value>) {
        if !self.properties.is_empty() {
            let mut props = Map::new();
            for (key, inferrer) in &self.properties {
                props.insert(key.clone(), inferrer.to_schema());
            }
            schema.insert("properties".to_string(), Value::Object(props));

            // Determine required properties (present in all observations)
            let required: Vec<String> = self
                .property_counts
                .iter()
                .filter(|(_, &count)| count == self.observation_count)
                .map(|(key, _)| key.clone())
                .collect();

            if !required.is_empty() {
                let mut sorted_required = required;
                sorted_required.sort();
                schema.insert("required".to_string(), json!(sorted_required));
            }
        }
    }
}

/// Detect common string formats.
fn detect_format(s: &str) -> Option<String> {
    // Email
    if s.contains('@') && s.contains('.') && !s.contains(' ') {
        return Some("email".to_string());
    }

    // UUID
    if s.len() == 36 && s.chars().filter(|&c| c == '-').count() == 4 {
        let parts: Vec<&str> = s.split('-').collect();
        if parts.len() == 5
            && parts[0].len() == 8
            && parts[1].len() == 4
            && parts[2].len() == 4
            && parts[3].len() == 4
            && parts[4].len() == 12
            && parts
                .iter()
                .all(|p| p.chars().all(|c| c.is_ascii_hexdigit()))
        {
            return Some("uuid".to_string());
        }
    }

    // ISO date-time
    if s.contains('T') && (s.ends_with('Z') || s.contains('+') || s.contains('-')) {
        if s.len() >= 19 && s.chars().nth(4) == Some('-') && s.chars().nth(7) == Some('-') {
            return Some("date-time".to_string());
        }
    }

    // ISO date
    if s.len() == 10
        && s.chars().nth(4) == Some('-')
        && s.chars().nth(7) == Some('-')
        && s.chars().filter(|c| c.is_ascii_digit()).count() == 8
    {
        return Some("date".to_string());
    }

    // Time
    if s.len() >= 8
        && s.chars().nth(2) == Some(':')
        && s.chars().nth(5) == Some(':')
        && s.chars().filter(|c| c.is_ascii_digit()).count() >= 6
    {
        return Some("time".to_string());
    }

    // URI/URL
    if s.starts_with("http://") || s.starts_with("https://") || s.starts_with("ftp://") {
        return Some("uri".to_string());
    }

    // IPv4
    let parts: Vec<&str> = s.split('.').collect();
    if parts.len() == 4 && parts.iter().all(|p| p.parse::<u8>().is_ok()) {
        return Some("ipv4".to_string());
    }

    // IPv6 (simplified check)
    if s.contains(':') && !s.contains('.') && s.len() >= 7 {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 3 && parts.iter().all(|p| p.is_empty() || p.len() <= 4) {
            return Some("ipv6".to_string());
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;

    #[derive(Serialize)]
    struct Person {
        name: String,
        age: u32,
        email: Option<String>,
    }

    #[test]
    fn test_infer_from_structs() {
        let records = vec![
            Person {
                name: "Alice".into(),
                age: 30,
                email: Some("alice@example.com".into()),
            },
            Person {
                name: "Bob".into(),
                age: 25,
                email: None,
            },
        ];

        let schema = infer_schema(&records);
        assert!(schema.get("properties").is_some());

        let props = schema.get("properties").unwrap();
        assert!(props.get("name").is_some());
        assert!(props.get("age").is_some());
    }

    #[test]
    fn test_infer_string() {
        let values = vec![json!("hello"), json!("world"), json!("test")];
        let schema = infer_schema_from_values(&values);
        assert_eq!(schema.get("type"), Some(&json!("string")));
    }

    #[test]
    fn test_infer_integer() {
        let values = vec![json!(1), json!(2), json!(3), json!(100)];
        let schema = infer_schema_from_values(&values);
        assert_eq!(schema.get("type"), Some(&json!("integer")));
        assert_eq!(schema.get("minimum"), Some(&json!(1)));
        assert_eq!(schema.get("maximum"), Some(&json!(100)));
    }

    #[test]
    fn test_infer_array() {
        let values = vec![json!([1, 2, 3]), json!([4, 5])];
        let schema = infer_schema_from_values(&values);
        assert_eq!(schema.get("type"), Some(&json!("array")));
        assert!(schema.get("items").is_some());
    }

    #[test]
    fn test_infer_enum() {
        let values = vec![
            json!("red"),
            json!("green"),
            json!("blue"),
            json!("red"),
            json!("green"),
        ];
        let schema = infer_schema_from_values(&values);
        assert!(schema.get("enum").is_some());
    }

    #[test]
    fn test_infer_email_format() {
        let values = vec![
            json!("alice@example.com"),
            json!("bob@test.org"),
            json!("carol@domain.net"),
        ];
        let schema = infer_schema_from_values(&values);
        assert_eq!(schema.get("format"), Some(&json!("email")));
    }

    #[test]
    fn test_infer_uuid_format() {
        let values = vec![
            json!("550e8400-e29b-41d4-a716-446655440000"),
            json!("6ba7b810-9dad-11d1-80b4-00c04fd430c8"),
        ];
        let schema = infer_schema_from_values(&values);
        assert_eq!(schema.get("format"), Some(&json!("uuid")));
    }

    #[test]
    fn test_infer_date_format() {
        let values = vec![json!("2024-01-15"), json!("2023-12-25")];
        let schema = infer_schema_from_values(&values);
        assert_eq!(schema.get("format"), Some(&json!("date")));
    }

    #[test]
    fn test_infer_required_properties() {
        let values = vec![
            json!({"name": "Alice", "age": 30}),
            json!({"name": "Bob", "age": 25}),
            json!({"name": "Carol", "age": 35, "email": "carol@test.com"}),
        ];
        let schema = infer_schema_from_values(&values);
        let required = schema.get("required").unwrap().as_array().unwrap();
        assert!(required.contains(&json!("name")));
        assert!(required.contains(&json!("age")));
        assert!(!required.contains(&json!("email")));
    }
}
