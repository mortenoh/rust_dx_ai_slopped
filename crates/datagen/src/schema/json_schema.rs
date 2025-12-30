//! JSON Schema to data generation.
//!
//! Generates random data that conforms to a JSON Schema definition.
//! Supports JSON Schema draft-07 and draft-2020-12 features.

use rand::Rng;
use serde_json::{json, Map, Value};
use std::collections::HashSet;

use crate::{
    alphanumeric, boolean, email, first_name, float_range, full_name, hex_string, int_range, ipv4,
    last_name, url, uuid::v4 as uuid_v4,
};

/// Options for JSON Schema data generation.
#[derive(Debug, Clone)]
pub struct JsonSchemaOptions {
    /// Maximum depth for recursive schemas (default: 5)
    pub max_depth: usize,
    /// Maximum items for arrays when maxItems is not specified (default: 5)
    pub default_max_items: usize,
    /// Minimum items for arrays when minItems is not specified (default: 1)
    pub default_min_items: usize,
    /// Maximum properties for additionalProperties (default: 3)
    pub default_max_additional_properties: usize,
    /// Default string length when not specified (default: 10)
    pub default_string_length: usize,
    /// Whether to include optional properties (default: true, 50% chance)
    pub include_optional: bool,
    /// Probability of including optional properties (0.0-1.0, default: 0.5)
    pub optional_probability: f64,
}

impl Default for JsonSchemaOptions {
    fn default() -> Self {
        Self {
            max_depth: 5,
            default_max_items: 5,
            default_min_items: 1,
            default_max_additional_properties: 3,
            default_string_length: 10,
            include_optional: true,
            optional_probability: 0.5,
        }
    }
}

/// Generate random data conforming to a JSON Schema.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `schema` - JSON Schema definition
///
/// # Returns
///
/// A `serde_json::Value` containing randomly generated data.
///
/// # Example
///
/// ```
/// use dx_datagen::schema::from_json_schema;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use serde_json::json;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "name": { "type": "string" },
///         "age": { "type": "integer", "minimum": 18, "maximum": 65 }
///     },
///     "required": ["name"]
/// });
///
/// let data = from_json_schema(&mut rng, &schema);
/// assert!(data.is_object());
/// ```
pub fn from_json_schema<R: Rng + ?Sized>(rng: &mut R, schema: &Value) -> Value {
    from_json_schema_with_options(rng, schema, &JsonSchemaOptions::default())
}

/// Generate random data conforming to a JSON Schema with custom options.
pub fn from_json_schema_with_options<R: Rng + ?Sized>(
    rng: &mut R,
    schema: &Value,
    options: &JsonSchemaOptions,
) -> Value {
    generate_value(rng, schema, options, 0, &mut HashSet::new())
}

fn generate_value<R: Rng + ?Sized>(
    rng: &mut R,
    schema: &Value,
    options: &JsonSchemaOptions,
    depth: usize,
    visited_refs: &mut HashSet<String>,
) -> Value {
    if depth > options.max_depth {
        return Value::Null;
    }

    // Handle boolean schemas
    if let Some(b) = schema.as_bool() {
        return if b {
            generate_any_value(rng, options)
        } else {
            Value::Null
        };
    }

    let obj = match schema.as_object() {
        Some(o) => o,
        None => return Value::Null,
    };

    // Handle $ref (simplified - doesn't resolve external refs)
    if let Some(ref_val) = obj.get("$ref") {
        if let Some(ref_str) = ref_val.as_str() {
            if visited_refs.contains(ref_str) {
                return Value::Null; // Avoid infinite recursion
            }
            visited_refs.insert(ref_str.to_string());
        }
    }

    // Handle const
    if let Some(const_val) = obj.get("const") {
        return const_val.clone();
    }

    // Handle enum
    if let Some(enum_val) = obj.get("enum") {
        if let Some(arr) = enum_val.as_array() {
            if !arr.is_empty() {
                let idx = rng.random_range(0..arr.len());
                return arr[idx].clone();
            }
        }
    }

    // Handle oneOf
    if let Some(one_of) = obj.get("oneOf") {
        if let Some(arr) = one_of.as_array() {
            if !arr.is_empty() {
                let idx = rng.random_range(0..arr.len());
                return generate_value(rng, &arr[idx], options, depth + 1, visited_refs);
            }
        }
    }

    // Handle anyOf (pick one randomly)
    if let Some(any_of) = obj.get("anyOf") {
        if let Some(arr) = any_of.as_array() {
            if !arr.is_empty() {
                let idx = rng.random_range(0..arr.len());
                return generate_value(rng, &arr[idx], options, depth + 1, visited_refs);
            }
        }
    }

    // Handle allOf (merge schemas)
    if let Some(all_of) = obj.get("allOf") {
        if let Some(arr) = all_of.as_array() {
            let mut merged = Map::new();
            for sub_schema in arr {
                if let Value::Object(sub_obj) =
                    generate_value(rng, sub_schema, options, depth + 1, visited_refs)
                {
                    for (k, v) in sub_obj {
                        merged.insert(k, v);
                    }
                }
            }
            return Value::Object(merged);
        }
    }

    // Get type(s)
    let types = get_types(obj);

    if types.is_empty() {
        // No type specified - try to infer from other keywords
        if obj.contains_key("properties")
            || obj.contains_key("additionalProperties")
            || obj.contains_key("patternProperties")
        {
            return generate_object(rng, obj, options, depth, visited_refs);
        }
        if obj.contains_key("items") || obj.contains_key("prefixItems") {
            return generate_array(rng, obj, options, depth, visited_refs);
        }
        if obj.contains_key("minimum")
            || obj.contains_key("maximum")
            || obj.contains_key("multipleOf")
        {
            return generate_number(rng, obj, false);
        }
        if obj.contains_key("minLength")
            || obj.contains_key("maxLength")
            || obj.contains_key("pattern")
            || obj.contains_key("format")
        {
            return generate_string(rng, obj, options);
        }
        return generate_any_value(rng, options);
    }

    // Pick a random type if multiple are specified
    let type_str = if types.len() == 1 {
        types[0].as_str()
    } else {
        let idx = rng.random_range(0..types.len());
        types[idx].as_str()
    };

    match type_str {
        "string" => generate_string(rng, obj, options),
        "integer" => generate_number(rng, obj, true),
        "number" => generate_number(rng, obj, false),
        "boolean" => Value::Bool(boolean(rng, 0.5)),
        "null" => Value::Null,
        "array" => generate_array(rng, obj, options, depth, visited_refs),
        "object" => generate_object(rng, obj, options, depth, visited_refs),
        _ => Value::Null,
    }
}

fn get_types(obj: &Map<String, Value>) -> Vec<String> {
    match obj.get("type") {
        Some(Value::String(s)) => vec![s.clone()],
        Some(Value::Array(arr)) => arr
            .iter()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
        _ => vec![],
    }
}

fn generate_string<R: Rng + ?Sized>(
    rng: &mut R,
    obj: &Map<String, Value>,
    options: &JsonSchemaOptions,
) -> Value {
    // Check format first
    if let Some(format) = obj.get("format").and_then(|v| v.as_str()) {
        return Value::String(generate_formatted_string(rng, format));
    }

    // Handle pattern (simplified regex support)
    if let Some(_pattern) = obj.get("pattern").and_then(|v| v.as_str()) {
        // For simplicity, just generate a random string
        // Full regex generation would require a regex-to-string generator
        let len = options.default_string_length;
        return Value::String(alphanumeric(rng, len));
    }

    let min_len = obj.get("minLength").and_then(|v| v.as_u64()).unwrap_or(1) as usize;
    let max_len = obj
        .get("maxLength")
        .and_then(|v| v.as_u64())
        .unwrap_or(options.default_string_length as u64) as usize;

    let len = if min_len >= max_len {
        min_len
    } else {
        rng.random_range(min_len..=max_len)
    };

    Value::String(alphanumeric(rng, len))
}

fn generate_formatted_string<R: Rng + ?Sized>(rng: &mut R, format: &str) -> String {
    match format {
        "email" => email(rng),
        "uri" | "url" => url(rng),
        "uuid" => uuid_v4().to_string(),
        "date" => {
            let year = rng.random_range(1970..2030);
            let month = rng.random_range(1..=12);
            let day = rng.random_range(1..=28);
            format!("{:04}-{:02}-{:02}", year, month, day)
        }
        "time" => {
            let hour = rng.random_range(0..24);
            let minute = rng.random_range(0..60);
            let second = rng.random_range(0..60);
            format!("{:02}:{:02}:{:02}", hour, minute, second)
        }
        "date-time" => {
            let year = rng.random_range(1970..2030);
            let month = rng.random_range(1..=12);
            let day = rng.random_range(1..=28);
            let hour = rng.random_range(0..24);
            let minute = rng.random_range(0..60);
            let second = rng.random_range(0..60);
            format!(
                "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
                year, month, day, hour, minute, second
            )
        }
        "ipv4" => ipv4(rng).to_string(),
        "ipv6" => {
            let segments: Vec<String> = (0..8).map(|_| hex_string(rng, 4)).collect();
            segments.join(":")
        }
        "hostname" => {
            format!("{}.example.com", alphanumeric(rng, 8).to_lowercase())
        }
        "idn-hostname" | "idn-email" => {
            // Simplified - just generate ASCII versions
            format!("{}.example.com", alphanumeric(rng, 8).to_lowercase())
        }
        "uri-reference" | "iri" | "iri-reference" => url(rng),
        "json-pointer" => format!("/{}/{}", alphanumeric(rng, 4), alphanumeric(rng, 4)),
        "relative-json-pointer" => format!("0/{}", alphanumeric(rng, 4)),
        "regex" => format!("^{}$", alphanumeric(rng, 5)),
        "phone" => {
            format!(
                "+1-{}-{}-{}",
                rng.random_range(200..999),
                rng.random_range(200..999),
                rng.random_range(1000..9999)
            )
        }
        "first-name" | "firstname" => first_name(rng).to_string(),
        "last-name" | "lastname" => last_name(rng).to_string(),
        "full-name" | "fullname" | "name" => full_name(rng),
        _ => alphanumeric(rng, 10),
    }
}

fn generate_number<R: Rng + ?Sized>(rng: &mut R, obj: &Map<String, Value>, integer: bool) -> Value {
    let min = obj
        .get("minimum")
        .and_then(|v| v.as_f64())
        .unwrap_or(if integer { i32::MIN as f64 } else { -1e6 });

    let max = obj
        .get("maximum")
        .and_then(|v| v.as_f64())
        .unwrap_or(if integer { i32::MAX as f64 } else { 1e6 });

    let exclusive_min = obj
        .get("exclusiveMinimum")
        .and_then(|v| v.as_f64())
        .map(|v| v + if integer { 1.0 } else { 0.0001 });

    let exclusive_max = obj
        .get("exclusiveMaximum")
        .and_then(|v| v.as_f64())
        .map(|v| v - if integer { 1.0 } else { 0.0001 });

    let actual_min = exclusive_min.unwrap_or(min);
    let actual_max = exclusive_max.unwrap_or(max);

    // Clamp to reasonable range
    let actual_min = actual_min.max(-1e9);
    let actual_max = actual_max.min(1e9);

    if integer {
        let min_i = actual_min.ceil() as i64;
        let max_i = actual_max.floor() as i64;
        if min_i >= max_i {
            return json!(min_i);
        }
        let val = int_range(rng, min_i, max_i);

        // Handle multipleOf
        if let Some(multiple) = obj.get("multipleOf").and_then(|v| v.as_i64()) {
            if multiple > 0 {
                let rounded = (val / multiple) * multiple;
                return json!(rounded);
            }
        }
        json!(val)
    } else {
        if actual_min >= actual_max {
            return json!(actual_min);
        }
        let val = float_range(rng, actual_min, actual_max);

        // Handle multipleOf for floats
        if let Some(multiple) = obj.get("multipleOf").and_then(|v| v.as_f64()) {
            if multiple > 0.0 {
                let rounded = (val / multiple).round() * multiple;
                return json!(rounded);
            }
        }
        json!(val)
    }
}

fn generate_array<R: Rng + ?Sized>(
    rng: &mut R,
    obj: &Map<String, Value>,
    options: &JsonSchemaOptions,
    depth: usize,
    visited_refs: &mut HashSet<String>,
) -> Value {
    let min_items = obj
        .get("minItems")
        .and_then(|v| v.as_u64())
        .unwrap_or(options.default_min_items as u64) as usize;

    let max_items = obj
        .get("maxItems")
        .and_then(|v| v.as_u64())
        .unwrap_or(options.default_max_items as u64) as usize;

    let count = if min_items >= max_items {
        min_items
    } else {
        rng.random_range(min_items..=max_items)
    };

    // Handle prefixItems (tuple validation)
    if let Some(prefix_items) = obj.get("prefixItems").and_then(|v| v.as_array()) {
        let mut arr: Vec<Value> = prefix_items
            .iter()
            .map(|item_schema| generate_value(rng, item_schema, options, depth + 1, visited_refs))
            .collect();

        // Add additional items if needed
        let additional_schema = obj.get("items").unwrap_or(&json!(true));
        while arr.len() < count {
            arr.push(generate_value(
                rng,
                additional_schema,
                options,
                depth + 1,
                visited_refs,
            ));
        }
        return Value::Array(arr);
    }

    // Handle items
    let empty_obj = json!({});
    let item_schema = obj.get("items").unwrap_or(&empty_obj);

    // Check for uniqueItems
    let unique_items = obj
        .get("uniqueItems")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    if unique_items {
        let mut arr = Vec::new();
        let mut seen = HashSet::new();
        let max_attempts = count * 10;
        let mut attempts = 0;

        while arr.len() < count && attempts < max_attempts {
            let val = generate_value(rng, item_schema, options, depth + 1, visited_refs);
            let key = val.to_string();
            if !seen.contains(&key) {
                seen.insert(key);
                arr.push(val);
            }
            attempts += 1;
        }
        Value::Array(arr)
    } else {
        let arr: Vec<Value> = (0..count)
            .map(|_| generate_value(rng, item_schema, options, depth + 1, visited_refs))
            .collect();
        Value::Array(arr)
    }
}

fn generate_object<R: Rng + ?Sized>(
    rng: &mut R,
    obj: &Map<String, Value>,
    options: &JsonSchemaOptions,
    depth: usize,
    visited_refs: &mut HashSet<String>,
) -> Value {
    let mut result = Map::new();

    // Get required properties
    let required: HashSet<String> = obj
        .get("required")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    // Generate defined properties
    if let Some(props) = obj.get("properties").and_then(|v| v.as_object()) {
        for (key, prop_schema) in props {
            let is_required = required.contains(key);
            let should_include = is_required
                || (options.include_optional
                    && rng.random_range(0.0..1.0) < options.optional_probability);

            if should_include {
                let val = generate_value(rng, prop_schema, options, depth + 1, visited_refs);
                result.insert(key.clone(), val);
            }
        }
    }

    // Handle additionalProperties
    if let Some(additional) = obj.get("additionalProperties") {
        if additional.as_bool() != Some(false) {
            let additional_schema = if additional.is_object() {
                additional
            } else {
                &json!({})
            };

            let min_props = obj
                .get("minProperties")
                .and_then(|v| v.as_u64())
                .unwrap_or(0) as usize;

            let current_count = result.len();
            if current_count < min_props {
                let needed = min_props - current_count;
                for i in 0..needed {
                    let key = format!("additional_{}", i);
                    let val =
                        generate_value(rng, additional_schema, options, depth + 1, visited_refs);
                    result.insert(key, val);
                }
            }
        }
    }

    Value::Object(result)
}

fn generate_any_value<R: Rng + ?Sized>(rng: &mut R, options: &JsonSchemaOptions) -> Value {
    match rng.random_range(0..6) {
        0 => Value::Null,
        1 => Value::Bool(boolean(rng, 0.5)),
        2 => json!(int_range(rng, -1000, 1000)),
        3 => json!(float_range(rng, -1000.0, 1000.0)),
        4 => Value::String(alphanumeric(rng, options.default_string_length)),
        _ => {
            let count = rng.random_range(0..3);
            let arr: Vec<Value> = (0..count)
                .map(|_| Value::String(alphanumeric(rng, 5)))
                .collect();
            Value::Array(arr)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    #[test]
    fn test_simple_string() {
        let mut rng = test_rng();
        let schema = json!({"type": "string"});
        let result = from_json_schema(&mut rng, &schema);
        assert!(result.is_string());
    }

    #[test]
    fn test_string_with_length() {
        let mut rng = test_rng();
        let schema = json!({"type": "string", "minLength": 5, "maxLength": 10});
        let result = from_json_schema(&mut rng, &schema);
        let s = result.as_str().unwrap();
        assert!(s.len() >= 5 && s.len() <= 10);
    }

    #[test]
    fn test_integer() {
        let mut rng = test_rng();
        let schema = json!({"type": "integer", "minimum": 10, "maximum": 20});
        let result = from_json_schema(&mut rng, &schema);
        let n = result.as_i64().unwrap();
        assert!(n >= 10 && n <= 20);
    }

    #[test]
    fn test_number() {
        let mut rng = test_rng();
        let schema = json!({"type": "number", "minimum": 0.0, "maximum": 1.0});
        let result = from_json_schema(&mut rng, &schema);
        let n = result.as_f64().unwrap();
        assert!(n >= 0.0 && n <= 1.0);
    }

    #[test]
    fn test_boolean() {
        let mut rng = test_rng();
        let schema = json!({"type": "boolean"});
        let result = from_json_schema(&mut rng, &schema);
        assert!(result.is_boolean());
    }

    #[test]
    fn test_enum() {
        let mut rng = test_rng();
        let schema = json!({"enum": ["red", "green", "blue"]});
        let result = from_json_schema(&mut rng, &schema);
        let s = result.as_str().unwrap();
        assert!(["red", "green", "blue"].contains(&s));
    }

    #[test]
    fn test_const() {
        let mut rng = test_rng();
        let schema = json!({"const": "fixed_value"});
        let result = from_json_schema(&mut rng, &schema);
        assert_eq!(result, json!("fixed_value"));
    }

    #[test]
    fn test_array() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "array",
            "items": {"type": "integer"},
            "minItems": 2,
            "maxItems": 5
        });
        let result = from_json_schema(&mut rng, &schema);
        let arr = result.as_array().unwrap();
        assert!(arr.len() >= 2 && arr.len() <= 5);
        assert!(arr.iter().all(|v| v.is_i64()));
    }

    #[test]
    fn test_object() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "age": {"type": "integer", "minimum": 0, "maximum": 120}
            },
            "required": ["name"]
        });
        let result = from_json_schema(&mut rng, &schema);
        let obj = result.as_object().unwrap();
        assert!(obj.contains_key("name"));
        assert!(obj.get("name").unwrap().is_string());
    }

    #[test]
    fn test_format_email() {
        let mut rng = test_rng();
        let schema = json!({"type": "string", "format": "email"});
        let result = from_json_schema(&mut rng, &schema);
        let s = result.as_str().unwrap();
        assert!(s.contains('@'));
    }

    #[test]
    fn test_format_uuid() {
        let mut rng = test_rng();
        let schema = json!({"type": "string", "format": "uuid"});
        let result = from_json_schema(&mut rng, &schema);
        let s = result.as_str().unwrap();
        assert_eq!(s.len(), 36); // UUID format: 8-4-4-4-12
    }

    #[test]
    fn test_one_of() {
        let mut rng = test_rng();
        let schema = json!({
            "oneOf": [
                {"type": "string"},
                {"type": "integer"}
            ]
        });
        let result = from_json_schema(&mut rng, &schema);
        assert!(result.is_string() || result.is_i64());
    }

    #[test]
    fn test_nested_object() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "object",
            "properties": {
                "person": {
                    "type": "object",
                    "properties": {
                        "name": {"type": "string"},
                        "address": {
                            "type": "object",
                            "properties": {
                                "city": {"type": "string"},
                                "zip": {"type": "string"}
                            }
                        }
                    }
                }
            }
        });
        let result = from_json_schema(&mut rng, &schema);
        assert!(result.is_object());
    }
}
