//! Avro schema support.
//!
//! Generate data from Apache Avro schema definitions.
//! This is a lightweight implementation that parses Avro schema JSON
//! and generates compatible data.

use rand::Rng;
use serde_json::{json, Map, Value};
use std::collections::HashSet;

use crate::{alphanumeric, boolean, float_range, hex_string, int_range};

/// Generate data from an Avro schema.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `schema` - Avro schema as JSON
///
/// # Returns
///
/// Generated data as JSON value.
///
/// # Example
///
/// ```
/// use dx_datagen::schema::from_avro_schema;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use serde_json::json;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let schema = json!({
///     "type": "record",
///     "name": "User",
///     "fields": [
///         { "name": "id", "type": "long" },
///         { "name": "name", "type": "string" },
///         { "name": "active", "type": "boolean" }
///     ]
/// });
///
/// let data = from_avro_schema(&mut rng, &schema);
/// assert!(data.is_object());
/// ```
pub fn from_avro_schema<R: Rng + ?Sized>(rng: &mut R, schema: &Value) -> Value {
    generate_avro_value(rng, schema, 0, &mut HashSet::new())
}

/// Convert a JSON Schema to an Avro schema.
///
/// # Arguments
///
/// * `json_schema` - JSON Schema definition
/// * `name` - Name for the record type
///
/// # Returns
///
/// Avro schema as JSON value.
pub fn to_avro_schema(json_schema: &Value, name: &str) -> Value {
    convert_to_avro(json_schema, name)
}

fn generate_avro_value<R: Rng + ?Sized>(
    rng: &mut R,
    schema: &Value,
    depth: usize,
    visited: &mut HashSet<String>,
) -> Value {
    if depth > 10 {
        return Value::Null;
    }

    // Handle string type names
    if let Some(type_str) = schema.as_str() {
        return generate_primitive(rng, type_str);
    }

    // Handle array (union type)
    if let Some(arr) = schema.as_array() {
        // Pick a non-null type if possible
        let non_null_types: Vec<&Value> = arr.iter().filter(|v| v != &&json!("null")).collect();
        if non_null_types.is_empty() {
            return Value::Null;
        }
        let idx = rng.random_range(0..non_null_types.len());
        return generate_avro_value(rng, non_null_types[idx], depth + 1, visited);
    }

    // Handle object schema
    let obj = match schema.as_object() {
        Some(o) => o,
        None => return Value::Null,
    };

    let type_val = match obj.get("type") {
        Some(t) => t,
        None => return Value::Null,
    };

    // Handle type as string
    if let Some(type_str) = type_val.as_str() {
        match type_str {
            "null" => Value::Null,
            "boolean" => Value::Bool(boolean(rng, 0.5)),
            "int" => json!(int_range(rng, i32::MIN as i64, i32::MAX as i64) as i32),
            "long" => json!(int_range(rng, -1_000_000_000, 1_000_000_000)),
            "float" => json!(float_range(rng, -1000.0, 1000.0) as f32),
            "double" => json!(float_range(rng, -1000.0, 1000.0)),
            "bytes" => {
                // Generate random bytes as base64-like string
                Value::String(hex_string(rng, 16))
            }
            "string" => {
                // Check for logical type
                if let Some(logical) = obj.get("logicalType").and_then(|v| v.as_str()) {
                    return generate_logical_type(rng, logical);
                }
                Value::String(alphanumeric(rng, 10))
            }
            "record" => generate_record(rng, obj, depth, visited),
            "enum" => generate_enum(rng, obj),
            "array" => generate_array(rng, obj, depth, visited),
            "map" => generate_map(rng, obj, depth, visited),
            "fixed" => generate_fixed(rng, obj),
            _ => Value::Null,
        }
    } else if let Some(type_arr) = type_val.as_array() {
        // Union type
        let non_null_types: Vec<&Value> =
            type_arr.iter().filter(|v| v != &&json!("null")).collect();
        if non_null_types.is_empty() {
            Value::Null
        } else {
            let idx = rng.random_range(0..non_null_types.len());
            generate_avro_value(rng, non_null_types[idx], depth + 1, visited)
        }
    } else {
        Value::Null
    }
}

fn generate_primitive<R: Rng + ?Sized>(rng: &mut R, type_str: &str) -> Value {
    match type_str {
        "null" => Value::Null,
        "boolean" => Value::Bool(boolean(rng, 0.5)),
        "int" => json!(int_range(rng, i32::MIN as i64, i32::MAX as i64) as i32),
        "long" => json!(int_range(rng, -1_000_000_000, 1_000_000_000)),
        "float" => json!(float_range(rng, -1000.0, 1000.0) as f32),
        "double" => json!(float_range(rng, -1000.0, 1000.0)),
        "bytes" => Value::String(hex_string(rng, 16)),
        "string" => Value::String(alphanumeric(rng, 10)),
        _ => Value::Null,
    }
}

fn generate_logical_type<R: Rng + ?Sized>(rng: &mut R, logical_type: &str) -> Value {
    match logical_type {
        "date" => {
            // Days since epoch
            json!(int_range(rng, 0, 20000))
        }
        "time-millis" => {
            // Milliseconds since midnight
            json!(int_range(rng, 0, 86_400_000))
        }
        "time-micros" => {
            // Microseconds since midnight
            json!(int_range(rng, 0, 86_400_000_000i64))
        }
        "timestamp-millis" => {
            // Milliseconds since epoch
            json!(int_range(rng, 0, 1_700_000_000_000i64))
        }
        "timestamp-micros" => {
            // Microseconds since epoch
            json!(int_range(rng, 0, 1_700_000_000_000_000i64))
        }
        "uuid" => Value::String(crate::uuid::v4().to_string()),
        "decimal" => {
            json!(float_range(rng, -10000.0, 10000.0))
        }
        _ => Value::String(alphanumeric(rng, 10)),
    }
}

fn generate_record<R: Rng + ?Sized>(
    rng: &mut R,
    obj: &Map<String, Value>,
    depth: usize,
    visited: &mut HashSet<String>,
) -> Value {
    // Check for recursive reference
    if let Some(name) = obj.get("name").and_then(|v| v.as_str()) {
        if visited.contains(name) {
            return Value::Null;
        }
        visited.insert(name.to_string());
    }

    let fields = match obj.get("fields").and_then(|f| f.as_array()) {
        Some(f) => f,
        None => return json!({}),
    };

    let mut result = Map::new();

    for field in fields {
        if let Some(field_obj) = field.as_object() {
            let name = field_obj
                .get("name")
                .and_then(|n| n.as_str())
                .unwrap_or("unknown");

            let null_type = json!("null");
            let field_type = field_obj.get("type").unwrap_or(&null_type);

            // Check for default value
            let value = if let Some(default) = field_obj.get("default") {
                // 50% chance to use default
                if boolean(rng, 0.5) {
                    default.clone()
                } else {
                    generate_avro_value(rng, field_type, depth + 1, visited)
                }
            } else {
                generate_avro_value(rng, field_type, depth + 1, visited)
            };

            result.insert(name.to_string(), value);
        }
    }

    Value::Object(result)
}

fn generate_enum<R: Rng + ?Sized>(rng: &mut R, obj: &Map<String, Value>) -> Value {
    let symbols = match obj.get("symbols").and_then(|s| s.as_array()) {
        Some(s) => s,
        None => return Value::Null,
    };

    if symbols.is_empty() {
        return Value::Null;
    }

    let idx = rng.random_range(0..symbols.len());
    symbols[idx].clone()
}

fn generate_array<R: Rng + ?Sized>(
    rng: &mut R,
    obj: &Map<String, Value>,
    depth: usize,
    visited: &mut HashSet<String>,
) -> Value {
    let items_schema = match obj.get("items") {
        Some(s) => s,
        None => return json!([]),
    };

    let count = rng.random_range(1..5);
    let arr: Vec<Value> = (0..count)
        .map(|_| generate_avro_value(rng, items_schema, depth + 1, visited))
        .collect();

    Value::Array(arr)
}

fn generate_map<R: Rng + ?Sized>(
    rng: &mut R,
    obj: &Map<String, Value>,
    depth: usize,
    visited: &mut HashSet<String>,
) -> Value {
    let values_schema = match obj.get("values") {
        Some(s) => s,
        None => return json!({}),
    };

    let count = rng.random_range(1..4);
    let mut result = Map::new();

    for i in 0..count {
        let key = format!("key_{}", i);
        let value = generate_avro_value(rng, values_schema, depth + 1, visited);
        result.insert(key, value);
    }

    Value::Object(result)
}

fn generate_fixed<R: Rng + ?Sized>(rng: &mut R, obj: &Map<String, Value>) -> Value {
    let size = obj.get("size").and_then(|s| s.as_u64()).unwrap_or(16) as usize;
    Value::String(hex_string(rng, size * 2)) // hex string is 2 chars per byte
}

fn convert_to_avro(json_schema: &Value, name: &str) -> Value {
    let obj = match json_schema.as_object() {
        Some(o) => o,
        None => return json!("null"),
    };

    let type_val = obj.get("type").and_then(|t| t.as_str()).unwrap_or("null");

    match type_val {
        "string" => {
            if let Some(format) = obj.get("format").and_then(|f| f.as_str()) {
                match format {
                    "date" => json!({"type": "int", "logicalType": "date"}),
                    "time" => json!({"type": "long", "logicalType": "time-micros"}),
                    "date-time" => json!({"type": "long", "logicalType": "timestamp-millis"}),
                    "uuid" => json!({"type": "string", "logicalType": "uuid"}),
                    _ => json!("string"),
                }
            } else {
                json!("string")
            }
        }
        "integer" => json!("long"),
        "number" => json!("double"),
        "boolean" => json!("boolean"),
        "null" => json!("null"),
        "array" => {
            let items = obj.get("items").map(|i| convert_to_avro(i, "item"));
            json!({
                "type": "array",
                "items": items.unwrap_or(json!("null"))
            })
        }
        "object" => {
            let mut fields = Vec::new();

            if let Some(props) = obj.get("properties").and_then(|p| p.as_object()) {
                let required: HashSet<&str> = obj
                    .get("required")
                    .and_then(|r| r.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
                    .unwrap_or_default();

                for (prop_name, prop_schema) in props {
                    let field_type = convert_to_avro(prop_schema, prop_name);
                    let is_required = required.contains(prop_name.as_str());

                    let final_type = if is_required {
                        field_type
                    } else {
                        // Make it a union with null
                        json!(["null", field_type])
                    };

                    fields.push(json!({
                        "name": prop_name,
                        "type": final_type
                    }));
                }
            }

            json!({
                "type": "record",
                "name": name,
                "fields": fields
            })
        }
        _ => json!("null"),
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
    fn test_primitive_types() {
        let mut rng = test_rng();

        assert!(from_avro_schema(&mut rng, &json!("null")).is_null());
        assert!(from_avro_schema(&mut rng, &json!("boolean")).is_boolean());
        assert!(from_avro_schema(&mut rng, &json!("int")).is_i64());
        assert!(from_avro_schema(&mut rng, &json!("long")).is_i64());
        assert!(from_avro_schema(&mut rng, &json!("float")).is_number());
        assert!(from_avro_schema(&mut rng, &json!("double")).is_number());
        assert!(from_avro_schema(&mut rng, &json!("string")).is_string());
    }

    #[test]
    fn test_record() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "record",
            "name": "User",
            "fields": [
                { "name": "id", "type": "long" },
                { "name": "name", "type": "string" },
                { "name": "active", "type": "boolean" }
            ]
        });

        let data = from_avro_schema(&mut rng, &schema);
        assert!(data.is_object());

        let obj = data.as_object().unwrap();
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
        assert!(obj.contains_key("active"));
    }

    #[test]
    fn test_enum() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "enum",
            "name": "Color",
            "symbols": ["RED", "GREEN", "BLUE"]
        });

        let data = from_avro_schema(&mut rng, &schema);
        assert!(data.is_string());
        let s = data.as_str().unwrap();
        assert!(["RED", "GREEN", "BLUE"].contains(&s));
    }

    #[test]
    fn test_array() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "array",
            "items": "string"
        });

        let data = from_avro_schema(&mut rng, &schema);
        assert!(data.is_array());
        assert!(data.as_array().unwrap().iter().all(|v| v.is_string()));
    }

    #[test]
    fn test_map() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "map",
            "values": "int"
        });

        let data = from_avro_schema(&mut rng, &schema);
        assert!(data.is_object());
    }

    #[test]
    fn test_union() {
        let mut rng = test_rng();
        let schema = json!(["null", "string"]);

        // Generate multiple times to ensure we get string sometimes
        let mut found_string = false;
        for _ in 0..20 {
            let data = from_avro_schema(&mut rng, &schema);
            if data.is_string() {
                found_string = true;
                break;
            }
        }
        assert!(found_string);
    }

    #[test]
    fn test_logical_types() {
        let mut rng = test_rng();

        let uuid_schema = json!({"type": "string", "logicalType": "uuid"});
        let uuid_data = from_avro_schema(&mut rng, &uuid_schema);
        assert!(uuid_data.is_string());
        assert_eq!(uuid_data.as_str().unwrap().len(), 36);

        let date_schema = json!({"type": "int", "logicalType": "date"});
        let date_data = from_avro_schema(&mut rng, &date_schema);
        assert!(date_data.is_i64());
    }

    #[test]
    fn test_fixed() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "fixed",
            "name": "md5",
            "size": 16
        });

        let data = from_avro_schema(&mut rng, &schema);
        assert!(data.is_string());
    }

    #[test]
    fn test_nested_record() {
        let mut rng = test_rng();
        let schema = json!({
            "type": "record",
            "name": "Person",
            "fields": [
                { "name": "name", "type": "string" },
                {
                    "name": "address",
                    "type": {
                        "type": "record",
                        "name": "Address",
                        "fields": [
                            { "name": "street", "type": "string" },
                            { "name": "city", "type": "string" }
                        ]
                    }
                }
            ]
        });

        let data = from_avro_schema(&mut rng, &schema);
        assert!(data.is_object());
        assert!(data.get("address").unwrap().is_object());
    }

    #[test]
    fn test_json_to_avro_conversion() {
        let json_schema = json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" },
                "tags": { "type": "array", "items": { "type": "string" } }
            },
            "required": ["id"]
        });

        let avro_schema = to_avro_schema(&json_schema, "MyRecord");
        assert_eq!(avro_schema.get("type"), Some(&json!("record")));
        assert_eq!(avro_schema.get("name"), Some(&json!("MyRecord")));

        let fields = avro_schema.get("fields").unwrap().as_array().unwrap();
        assert!(!fields.is_empty());
    }
}
