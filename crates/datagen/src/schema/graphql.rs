//! GraphQL mock data generation.
//!
//! Generate mock data from GraphQL type definitions.
//! This is a lightweight parser that handles common GraphQL SDL syntax.

use rand::Rng;
use serde_json::{json, Map, Value};
use std::collections::HashMap;

use crate::{alphanumeric, boolean, email, float_range, int_range, ipv4, url, uuid::v4 as uuid_v4};

/// Parsed GraphQL type.
#[derive(Debug, Clone)]
enum GraphQLType {
    Scalar(String),
    Object(String),
    List(Box<GraphQLType>),
    NonNull(Box<GraphQLType>),
    Enum(Vec<String>),
}

/// Parsed GraphQL field.
#[derive(Debug, Clone)]
struct GraphQLField {
    name: String,
    field_type: GraphQLType,
}

/// Parsed GraphQL type definition.
#[derive(Debug, Clone)]
struct TypeDefinition {
    name: String,
    fields: Vec<GraphQLField>,
}

/// Parsed GraphQL enum definition.
#[derive(Debug, Clone)]
struct EnumDefinition {
    name: String,
    values: Vec<String>,
}

/// Parsed GraphQL schema.
#[derive(Debug, Clone, Default)]
pub struct GraphQLSchema {
    types: HashMap<String, TypeDefinition>,
    enums: HashMap<String, EnumDefinition>,
    query_type: Option<String>,
    mutation_type: Option<String>,
}

impl GraphQLSchema {
    /// Parse a GraphQL SDL schema string.
    pub fn parse(sdl: &str) -> Result<Self, String> {
        let mut schema = GraphQLSchema::default();

        // Remove comments
        let cleaned = remove_comments(sdl);

        // Simple tokenization and parsing
        let mut current_pos = 0;

        while current_pos < cleaned.len() {
            skip_whitespace(&cleaned, &mut current_pos);
            if current_pos >= cleaned.len() {
                break;
            }

            let remaining = &cleaned[current_pos..];

            if remaining.starts_with("type ") {
                let (type_def, consumed) = parse_type_definition(remaining)?;
                if type_def.name == "Query" {
                    schema.query_type = Some("Query".to_string());
                } else if type_def.name == "Mutation" {
                    schema.mutation_type = Some("Mutation".to_string());
                }
                schema.types.insert(type_def.name.clone(), type_def);
                current_pos += consumed;
            } else if remaining.starts_with("enum ") {
                let (enum_def, consumed) = parse_enum_definition(remaining)?;
                schema.enums.insert(enum_def.name.clone(), enum_def);
                current_pos += consumed;
            } else if remaining.starts_with("input ") {
                // Treat input types like regular types
                let remaining_mod = remaining.replacen("input ", "type ", 1);
                let (type_def, consumed) = parse_type_definition(&remaining_mod)?;
                schema.types.insert(type_def.name.clone(), type_def);
                current_pos += consumed;
            } else if remaining.starts_with("interface ")
                || remaining.starts_with("scalar ")
                || remaining.starts_with("union ")
                || remaining.starts_with("schema ")
                || remaining.starts_with("directive ")
                || remaining.starts_with("extend ")
            {
                // Skip to next definition
                if let Some(idx) = remaining.find('}') {
                    current_pos += idx + 1;
                } else {
                    // Skip to end of line for scalar, etc.
                    if let Some(idx) = remaining.find('\n') {
                        current_pos += idx + 1;
                    } else {
                        break;
                    }
                }
            } else {
                current_pos += 1;
            }
        }

        Ok(schema)
    }

    /// Get a type definition by name.
    pub fn get_type(&self, name: &str) -> Option<&TypeDefinition> {
        self.types.get(name)
    }

    /// Get an enum definition by name.
    pub fn get_enum(&self, name: &str) -> Option<&EnumDefinition> {
        self.enums.get(name)
    }
}

/// Generate mock data from a GraphQL schema for a specific type.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `schema_sdl` - GraphQL SDL schema string
/// * `type_name` - Name of the type to generate data for
///
/// # Returns
///
/// Generated mock data as JSON.
///
/// # Example
///
/// ```
/// use dx_datagen::schema::from_graphql_schema;
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
///
/// let schema = r#"
///     type User {
///         id: ID!
///         name: String!
///         email: String
///         posts: [Post!]!
///     }
///
///     type Post {
///         id: ID!
///         title: String!
///         content: String
///     }
/// "#;
///
/// let mut rng = StdRng::seed_from_u64(42);
/// let user = from_graphql_schema(&mut rng, schema, "User");
/// assert!(user.is_ok());
/// ```
pub fn from_graphql_schema<R: Rng + ?Sized>(
    rng: &mut R,
    schema_sdl: &str,
    type_name: &str,
) -> Result<Value, String> {
    let schema = GraphQLSchema::parse(schema_sdl)?;
    Ok(generate_type(rng, &schema, type_name, 0))
}

/// Generate mock data from a GraphQL query.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `schema_sdl` - GraphQL SDL schema string
/// * `query` - GraphQL query string
///
/// # Returns
///
/// Generated mock response data as JSON.
pub fn from_graphql_query<R: Rng + ?Sized>(
    rng: &mut R,
    schema_sdl: &str,
    query: &str,
) -> Result<Value, String> {
    let schema = GraphQLSchema::parse(schema_sdl)?;

    // Simple query parsing - extract field selections
    let fields = parse_query_fields(query)?;

    let mut result = Map::new();
    result.insert(
        "data".to_string(),
        generate_query_result(rng, &schema, &fields, 0),
    );

    Ok(Value::Object(result))
}

fn remove_comments(sdl: &str) -> String {
    let mut result = String::new();
    let mut in_string = false;
    let mut chars = sdl.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '"' {
            in_string = !in_string;
            result.push(c);
        } else if !in_string && c == '#' {
            // Skip until newline
            while let Some(&next) = chars.peek() {
                if next == '\n' {
                    break;
                }
                chars.next();
            }
        } else {
            result.push(c);
        }
    }

    result
}

fn skip_whitespace(s: &str, pos: &mut usize) {
    while *pos < s.len() {
        let c = s.chars().nth(*pos).unwrap_or('\0');
        if c.is_whitespace() {
            *pos += 1;
        } else {
            break;
        }
    }
}

fn parse_type_definition(s: &str) -> Result<(TypeDefinition, usize), String> {
    // Find type name
    let type_start = s.find("type ").ok_or("Expected 'type' keyword")? + 5;
    let mut pos = type_start;

    // Skip whitespace
    while pos < s.len()
        && s.chars()
            .nth(pos)
            .map(|c| c.is_whitespace())
            .unwrap_or(false)
    {
        pos += 1;
    }

    // Read type name
    let name_start = pos;
    while pos < s.len()
        && s.chars()
            .nth(pos)
            .map(|c| c.is_alphanumeric() || c == '_')
            .unwrap_or(false)
    {
        pos += 1;
    }
    let type_name = s[name_start..pos].to_string();

    // Find opening brace
    let brace_start = s[pos..].find('{').ok_or("Expected '{'")?;
    pos += brace_start + 1;

    // Find closing brace
    let brace_end = find_matching_brace(&s[pos - 1..]).ok_or("Expected '}'")?;
    let fields_str = &s[pos..pos + brace_end - 1];

    // Parse fields
    let fields = parse_fields(fields_str)?;

    Ok((
        TypeDefinition {
            name: type_name,
            fields,
        },
        pos + brace_end,
    ))
}

fn parse_enum_definition(s: &str) -> Result<(EnumDefinition, usize), String> {
    let enum_start = s.find("enum ").ok_or("Expected 'enum' keyword")? + 5;
    let mut pos = enum_start;

    // Skip whitespace
    while pos < s.len()
        && s.chars()
            .nth(pos)
            .map(|c| c.is_whitespace())
            .unwrap_or(false)
    {
        pos += 1;
    }

    // Read enum name
    let name_start = pos;
    while pos < s.len()
        && s.chars()
            .nth(pos)
            .map(|c| c.is_alphanumeric() || c == '_')
            .unwrap_or(false)
    {
        pos += 1;
    }
    let enum_name = s[name_start..pos].to_string();

    // Find opening brace
    let brace_start = s[pos..].find('{').ok_or("Expected '{'")?;
    pos += brace_start + 1;

    // Find closing brace
    let brace_end = find_matching_brace(&s[pos - 1..]).ok_or("Expected '}'")?;
    let values_str = &s[pos..pos + brace_end - 1];

    // Parse enum values
    let values: Vec<String> = values_str
        .split_whitespace()
        .filter(|s| !s.is_empty())
        .map(|s| s.trim().to_string())
        .collect();

    Ok((
        EnumDefinition {
            name: enum_name,
            values,
        },
        pos + brace_end,
    ))
}

fn find_matching_brace(s: &str) -> Option<usize> {
    let mut depth = 0;
    for (i, c) in s.chars().enumerate() {
        match c {
            '{' => depth += 1,
            '}' => {
                depth -= 1;
                if depth == 0 {
                    return Some(i);
                }
            }
            _ => {}
        }
    }
    None
}

fn parse_fields(s: &str) -> Result<Vec<GraphQLField>, String> {
    let mut fields = Vec::new();

    for line in s.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Skip arguments for now (find the part after parentheses)
        let line = if let Some(paren_end) = line.find(')') {
            &line[paren_end + 1..]
        } else {
            line
        };

        // Parse field: name: Type
        if let Some(colon_pos) = line.find(':') {
            let name = line[..colon_pos].trim();
            // Remove any arguments before the colon
            let name = if let Some(paren_pos) = name.find('(') {
                &name[..paren_pos]
            } else {
                name
            };
            let name = name.trim().to_string();

            if name.is_empty() {
                continue;
            }

            let type_str = line[colon_pos + 1..].trim();
            let field_type = parse_type(type_str)?;

            fields.push(GraphQLField { name, field_type });
        }
    }

    Ok(fields)
}

fn parse_type(s: &str) -> Result<GraphQLType, String> {
    let s = s.trim();

    if s.ends_with('!') {
        let inner = parse_type(&s[..s.len() - 1])?;
        return Ok(GraphQLType::NonNull(Box::new(inner)));
    }

    if s.starts_with('[') && s.ends_with(']') {
        let inner = parse_type(&s[1..s.len() - 1])?;
        return Ok(GraphQLType::List(Box::new(inner)));
    }

    // Handle inline enum-like syntax or just type name
    Ok(match s {
        "ID" | "String" | "Int" | "Float" | "Boolean" | "DateTime" | "Date" | "Time" | "JSON" => {
            GraphQLType::Scalar(s.to_string())
        }
        _ => GraphQLType::Object(s.to_string()),
    })
}

fn generate_type<R: Rng + ?Sized>(
    rng: &mut R,
    schema: &GraphQLSchema,
    type_name: &str,
    depth: usize,
) -> Value {
    if depth > 5 {
        return Value::Null;
    }

    // Check if it's a scalar
    if is_scalar(type_name) {
        return generate_scalar(rng, type_name);
    }

    // Check if it's an enum
    if let Some(enum_def) = schema.get_enum(type_name) {
        if enum_def.values.is_empty() {
            return Value::Null;
        }
        let idx = rng.random_range(0..enum_def.values.len());
        return json!(enum_def.values[idx]);
    }

    // Generate object type
    let type_def = match schema.get_type(type_name) {
        Some(t) => t,
        None => return Value::Null,
    };

    let mut result = Map::new();

    for field in &type_def.fields {
        let value = generate_field_value(rng, schema, &field.field_type, depth + 1);
        result.insert(field.name.clone(), value);
    }

    Value::Object(result)
}

fn generate_field_value<R: Rng + ?Sized>(
    rng: &mut R,
    schema: &GraphQLSchema,
    field_type: &GraphQLType,
    depth: usize,
) -> Value {
    match field_type {
        GraphQLType::Scalar(name) => generate_scalar(rng, name),
        GraphQLType::Object(name) => {
            // Check if it's an enum
            if let Some(enum_def) = schema.get_enum(name) {
                if enum_def.values.is_empty() {
                    Value::Null
                } else {
                    let idx = rng.random_range(0..enum_def.values.len());
                    json!(enum_def.values[idx])
                }
            } else {
                generate_type(rng, schema, name, depth)
            }
        }
        GraphQLType::List(inner) => {
            let count = rng.random_range(1..4);
            let arr: Vec<Value> = (0..count)
                .map(|_| generate_field_value(rng, schema, inner, depth))
                .collect();
            Value::Array(arr)
        }
        GraphQLType::NonNull(inner) => generate_field_value(rng, schema, inner, depth),
        GraphQLType::Enum(values) => {
            if values.is_empty() {
                Value::Null
            } else {
                let idx = rng.random_range(0..values.len());
                json!(values[idx])
            }
        }
    }
}

fn is_scalar(name: &str) -> bool {
    matches!(
        name,
        "ID" | "String"
            | "Int"
            | "Float"
            | "Boolean"
            | "DateTime"
            | "Date"
            | "Time"
            | "JSON"
            | "UUID"
            | "Email"
            | "URL"
            | "IP"
    )
}

fn generate_scalar<R: Rng + ?Sized>(rng: &mut R, name: &str) -> Value {
    match name {
        "ID" => json!(uuid_v4().to_string()),
        "String" => json!(alphanumeric(rng, 10)),
        "Int" => json!(int_range(rng, -1000, 1000)),
        "Float" => json!(float_range(rng, -1000.0, 1000.0)),
        "Boolean" => json!(boolean(rng, 0.5)),
        "DateTime" => {
            let year = rng.random_range(2020..2025);
            let month = rng.random_range(1..=12);
            let day = rng.random_range(1..=28);
            json!(format!("{:04}-{:02}-{:02}T12:00:00Z", year, month, day))
        }
        "Date" => {
            let year = rng.random_range(2020..2025);
            let month = rng.random_range(1..=12);
            let day = rng.random_range(1..=28);
            json!(format!("{:04}-{:02}-{:02}", year, month, day))
        }
        "Time" => {
            let hour = rng.random_range(0..24);
            let minute = rng.random_range(0..60);
            json!(format!("{:02}:{:02}:00", hour, minute))
        }
        "JSON" => json!({"key": "value"}),
        "UUID" => json!(uuid_v4().to_string()),
        "Email" => json!(email(rng)),
        "URL" => json!(url(rng)),
        "IP" => json!(ipv4(rng).to_string()),
        _ => json!(alphanumeric(rng, 10)),
    }
}

fn parse_query_fields(query: &str) -> Result<Vec<QueryField>, String> {
    // Very simple query parser - just extracts top-level selections
    let query = query.trim();

    // Find the first { and match }
    let start = query.find('{').ok_or("Expected '{' in query")?;
    let end = find_matching_brace(&query[start..]).ok_or("Expected '}' in query")?;

    let body = &query[start + 1..start + end];

    // Parse field selections
    let mut fields = Vec::new();
    for line in body.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Simple field name extraction
        let name = line
            .split(|c: char| !c.is_alphanumeric() && c != '_')
            .next()
            .unwrap_or("")
            .to_string();

        if !name.is_empty() {
            fields.push(QueryField {
                name,
                type_name: None,
            });
        }
    }

    Ok(fields)
}

#[derive(Debug)]
struct QueryField {
    name: String,
    type_name: Option<String>,
}

fn generate_query_result<R: Rng + ?Sized>(
    rng: &mut R,
    schema: &GraphQLSchema,
    fields: &[QueryField],
    depth: usize,
) -> Value {
    let mut result = Map::new();

    // Try to find fields in Query type
    if let Some(query_type) = schema.get_type("Query") {
        for field in fields {
            if let Some(schema_field) = query_type.fields.iter().find(|f| f.name == field.name) {
                let value = generate_field_value(rng, schema, &schema_field.field_type, depth);
                result.insert(field.name.clone(), value);
            } else {
                // Generate generic data
                result.insert(field.name.clone(), json!(alphanumeric(rng, 10)));
            }
        }
    }

    Value::Object(result)
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
    fn test_parse_simple_schema() {
        let schema = r#"
            type User {
                id: ID!
                name: String!
                age: Int
            }
        "#;

        let parsed = GraphQLSchema::parse(schema).unwrap();
        assert!(parsed.types.contains_key("User"));

        let user_type = parsed.get_type("User").unwrap();
        assert_eq!(user_type.fields.len(), 3);
    }

    #[test]
    fn test_parse_enum() {
        let schema = r#"
            enum Status {
                ACTIVE
                INACTIVE
                PENDING
            }
        "#;

        let parsed = GraphQLSchema::parse(schema).unwrap();
        assert!(parsed.enums.contains_key("Status"));

        let status_enum = parsed.get_enum("Status").unwrap();
        assert_eq!(status_enum.values.len(), 3);
    }

    #[test]
    fn test_generate_simple_type() {
        let schema = r#"
            type User {
                id: ID!
                name: String!
                email: String
            }
        "#;

        let mut rng = test_rng();
        let data = from_graphql_schema(&mut rng, schema, "User").unwrap();

        assert!(data.is_object());
        let obj = data.as_object().unwrap();
        assert!(obj.contains_key("id"));
        assert!(obj.contains_key("name"));
    }

    #[test]
    fn test_generate_nested_type() {
        let schema = r#"
            type User {
                id: ID!
                name: String!
                posts: [Post!]!
            }

            type Post {
                id: ID!
                title: String!
            }
        "#;

        let mut rng = test_rng();
        let data = from_graphql_schema(&mut rng, schema, "User").unwrap();

        assert!(data.is_object());
        let posts = data.get("posts").unwrap();
        assert!(posts.is_array());
    }

    #[test]
    fn test_generate_with_enum() {
        let schema = r#"
            enum Status {
                ACTIVE
                INACTIVE
            }

            type User {
                id: ID!
                status: Status!
            }
        "#;

        let mut rng = test_rng();
        let data = from_graphql_schema(&mut rng, schema, "User").unwrap();

        let status = data.get("status").unwrap().as_str().unwrap();
        assert!(status == "ACTIVE" || status == "INACTIVE");
    }

    #[test]
    fn test_generate_query_response() {
        let schema = r#"
            type Query {
                user: User
                users: [User!]!
            }

            type User {
                id: ID!
                name: String!
            }
        "#;

        let query = r#"
            query {
                user
                users
            }
        "#;

        let mut rng = test_rng();
        let response = from_graphql_query(&mut rng, schema, query).unwrap();

        assert!(response.get("data").is_some());
    }

    #[test]
    fn test_scalars() {
        let schema = r#"
            type Test {
                id: ID!
                str: String!
                num: Int!
                fl: Float!
                bool: Boolean!
            }
        "#;

        let mut rng = test_rng();
        let data = from_graphql_schema(&mut rng, schema, "Test").unwrap();

        assert!(data.get("id").unwrap().is_string());
        assert!(data.get("str").unwrap().is_string());
        assert!(data.get("num").unwrap().is_i64());
        assert!(data.get("fl").unwrap().is_number());
        assert!(data.get("bool").unwrap().is_boolean());
    }

    #[test]
    fn test_list_type() {
        let schema = r#"
            type User {
                tags: [String!]!
            }
        "#;

        let mut rng = test_rng();
        let data = from_graphql_schema(&mut rng, schema, "User").unwrap();

        let tags = data.get("tags").unwrap();
        assert!(tags.is_array());
        assert!(!tags.as_array().unwrap().is_empty());
    }
}
