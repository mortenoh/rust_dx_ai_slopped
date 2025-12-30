//! OpenAPI/Swagger mock data generation.
//!
//! Generate mock API responses from OpenAPI 3.x specifications.

use rand::Rng;
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::collections::HashMap;

use super::json_schema::{from_json_schema_with_options, JsonSchemaOptions};

/// Simplified OpenAPI specification structure.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenApiSpec {
    /// OpenAPI version (e.g., "3.0.0", "3.1.0")
    #[serde(default)]
    pub openapi: String,
    /// API information
    #[serde(default)]
    pub info: ApiInfo,
    /// Available paths/endpoints
    #[serde(default)]
    pub paths: HashMap<String, PathItem>,
    /// Reusable component schemas
    #[serde(default)]
    pub components: Option<Components>,
}

/// API information.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiInfo {
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub description: Option<String>,
}

/// Path item containing operations.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PathItem {
    #[serde(default)]
    pub get: Option<Operation>,
    #[serde(default)]
    pub post: Option<Operation>,
    #[serde(default)]
    pub put: Option<Operation>,
    #[serde(default)]
    pub delete: Option<Operation>,
    #[serde(default)]
    pub patch: Option<Operation>,
    #[serde(default)]
    pub head: Option<Operation>,
    #[serde(default)]
    pub options: Option<Operation>,
}

/// API operation (endpoint).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Operation {
    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(rename = "operationId")]
    #[serde(default)]
    pub operation_id: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(rename = "requestBody")]
    #[serde(default)]
    pub request_body: Option<RequestBody>,
    #[serde(default)]
    pub responses: HashMap<String, Response>,
}

/// Operation parameter.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(default)]
    pub name: String,
    #[serde(rename = "in")]
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub schema: Option<Value>,
}

/// Request body definition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
}

/// Response definition.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub content: Option<HashMap<String, MediaType>>,
}

/// Media type with schema.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaType {
    #[serde(default)]
    pub schema: Option<Value>,
    #[serde(default)]
    pub example: Option<Value>,
    #[serde(default)]
    pub examples: Option<HashMap<String, Example>>,
}

/// Example value.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Example {
    #[serde(default)]
    pub value: Option<Value>,
    #[serde(default)]
    pub summary: Option<String>,
}

/// Reusable components.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Components {
    #[serde(default)]
    pub schemas: HashMap<String, Value>,
    #[serde(default)]
    pub responses: HashMap<String, Response>,
    #[serde(default)]
    pub parameters: HashMap<String, Parameter>,
    #[serde(default)]
    pub examples: HashMap<String, Example>,
}

impl OpenApiSpec {
    /// Parse an OpenAPI spec from JSON.
    pub fn from_json(json: &Value) -> Result<Self, serde_json::Error> {
        serde_json::from_value(json.clone())
    }

    /// Parse an OpenAPI spec from a JSON string.
    pub fn from_str(s: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(s)
    }

    /// Get operation for a path and method.
    pub fn get_operation(&self, path: &str, method: &str) -> Option<&Operation> {
        let path_item = self.paths.get(path)?;
        match method.to_lowercase().as_str() {
            "get" => path_item.get.as_ref(),
            "post" => path_item.post.as_ref(),
            "put" => path_item.put.as_ref(),
            "delete" => path_item.delete.as_ref(),
            "patch" => path_item.patch.as_ref(),
            "head" => path_item.head.as_ref(),
            "options" => path_item.options.as_ref(),
            _ => None,
        }
    }

    /// Resolve a $ref reference.
    fn resolve_ref(&self, ref_str: &str) -> Option<Value> {
        if !ref_str.starts_with("#/components/schemas/") {
            return None;
        }

        let schema_name = ref_str.strip_prefix("#/components/schemas/")?;
        self.components
            .as_ref()
            .and_then(|c| c.schemas.get(schema_name))
            .cloned()
    }

    /// Resolve all $ref references in a schema.
    fn resolve_schema(&self, schema: &Value) -> Value {
        match schema {
            Value::Object(obj) => {
                // Check for $ref
                if let Some(ref_val) = obj.get("$ref") {
                    if let Some(ref_str) = ref_val.as_str() {
                        if let Some(resolved) = self.resolve_ref(ref_str) {
                            return self.resolve_schema(&resolved);
                        }
                    }
                }

                // Recursively resolve nested objects
                let mut new_obj = Map::new();
                for (key, val) in obj {
                    new_obj.insert(key.clone(), self.resolve_schema(val));
                }
                Value::Object(new_obj)
            }
            Value::Array(arr) => Value::Array(arr.iter().map(|v| self.resolve_schema(v)).collect()),
            _ => schema.clone(),
        }
    }
}

/// Generate mock response data from an OpenAPI spec.
///
/// # Arguments
///
/// * `rng` - Random number generator
/// * `spec` - OpenAPI specification
/// * `path` - API path (e.g., "/users/{id}")
/// * `method` - HTTP method (e.g., "GET")
/// * `status_code` - HTTP status code (e.g., "200", "default")
///
/// # Returns
///
/// Generated mock response data as JSON.
///
/// # Example
///
/// ```
/// use dx_datagen::schema::{from_openapi, OpenApiSpec};
/// use rand::SeedableRng;
/// use rand::rngs::StdRng;
/// use serde_json::json;
///
/// let spec_json = json!({
///     "openapi": "3.0.0",
///     "info": { "title": "Test API", "version": "1.0.0" },
///     "paths": {
///         "/users": {
///             "get": {
///                 "responses": {
///                     "200": {
///                         "description": "Success",
///                         "content": {
///                             "application/json": {
///                                 "schema": {
///                                     "type": "array",
///                                     "items": {
///                                         "type": "object",
///                                         "properties": {
///                                             "id": { "type": "integer" },
///                                             "name": { "type": "string" }
///                                         }
///                                     }
///                                 }
///                             }
///                         }
///                     }
///                 }
///             }
///         }
///     }
/// });
///
/// let spec = OpenApiSpec::from_json(&spec_json).unwrap();
/// let mut rng = StdRng::seed_from_u64(42);
///
/// let response = from_openapi(&mut rng, &spec, "/users", "GET", "200");
/// assert!(response.is_some());
/// ```
pub fn from_openapi<R: Rng + ?Sized>(
    rng: &mut R,
    spec: &OpenApiSpec,
    path: &str,
    method: &str,
    status_code: &str,
) -> Option<Value> {
    let operation = spec.get_operation(path, method)?;
    let response = operation
        .responses
        .get(status_code)
        .or_else(|| operation.responses.get("default"))?;

    let content = response.content.as_ref()?;

    // Try common content types
    let media_type = content
        .get("application/json")
        .or_else(|| content.get("*/*"))
        .or_else(|| content.values().next())?;

    // Check for example first
    if let Some(example) = &media_type.example {
        return Some(example.clone());
    }

    // Check for named examples
    if let Some(examples) = &media_type.examples {
        if let Some(first) = examples.values().next() {
            if let Some(value) = &first.value {
                return Some(value.clone());
            }
        }
    }

    // Generate from schema
    let schema = media_type.schema.as_ref()?;
    let resolved_schema = spec.resolve_schema(schema);

    let options = JsonSchemaOptions {
        default_max_items: 3,
        default_min_items: 1,
        ..Default::default()
    };

    Some(from_json_schema_with_options(
        rng,
        &resolved_schema,
        &options,
    ))
}

/// Generate mock request body data from an OpenAPI spec.
pub fn generate_request_body<R: Rng + ?Sized>(
    rng: &mut R,
    spec: &OpenApiSpec,
    path: &str,
    method: &str,
) -> Option<Value> {
    let operation = spec.get_operation(path, method)?;
    let request_body = operation.request_body.as_ref()?;

    // Try common content types
    let media_type = request_body
        .content
        .get("application/json")
        .or_else(|| request_body.content.get("*/*"))
        .or_else(|| request_body.content.values().next())?;

    // Check for example first
    if let Some(example) = &media_type.example {
        return Some(example.clone());
    }

    // Generate from schema
    let schema = media_type.schema.as_ref()?;
    let resolved_schema = spec.resolve_schema(schema);

    Some(from_json_schema_with_options(
        rng,
        &resolved_schema,
        &JsonSchemaOptions::default(),
    ))
}

/// Generate mock parameter values from an OpenAPI spec.
pub fn generate_parameters<R: Rng + ?Sized>(
    rng: &mut R,
    spec: &OpenApiSpec,
    path: &str,
    method: &str,
) -> HashMap<String, Value> {
    let mut params = HashMap::new();

    let operation = match spec.get_operation(path, method) {
        Some(op) => op,
        None => return params,
    };

    for param in &operation.parameters {
        if let Some(schema) = &param.schema {
            let resolved = spec.resolve_schema(schema);
            let value =
                from_json_schema_with_options(rng, &resolved, &JsonSchemaOptions::default());
            params.insert(param.name.clone(), value);
        }
    }

    params
}

/// List all available paths and methods in the spec.
pub fn list_endpoints(spec: &OpenApiSpec) -> Vec<(String, String)> {
    let mut endpoints = Vec::new();

    for (path, path_item) in &spec.paths {
        if path_item.get.is_some() {
            endpoints.push((path.clone(), "GET".to_string()));
        }
        if path_item.post.is_some() {
            endpoints.push((path.clone(), "POST".to_string()));
        }
        if path_item.put.is_some() {
            endpoints.push((path.clone(), "PUT".to_string()));
        }
        if path_item.delete.is_some() {
            endpoints.push((path.clone(), "DELETE".to_string()));
        }
        if path_item.patch.is_some() {
            endpoints.push((path.clone(), "PATCH".to_string()));
        }
    }

    endpoints
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use serde_json::json;

    fn test_rng() -> ChaCha8Rng {
        ChaCha8Rng::seed_from_u64(42)
    }

    fn sample_spec() -> OpenApiSpec {
        let json = json!({
            "openapi": "3.0.0",
            "info": { "title": "Test API", "version": "1.0.0" },
            "paths": {
                "/users": {
                    "get": {
                        "operationId": "getUsers",
                        "responses": {
                            "200": {
                                "description": "Success",
                                "content": {
                                    "application/json": {
                                        "schema": {
                                            "type": "array",
                                            "items": { "$ref": "#/components/schemas/User" }
                                        }
                                    }
                                }
                            }
                        }
                    },
                    "post": {
                        "operationId": "createUser",
                        "requestBody": {
                            "required": true,
                            "content": {
                                "application/json": {
                                    "schema": { "$ref": "#/components/schemas/User" }
                                }
                            }
                        },
                        "responses": {
                            "201": {
                                "description": "Created",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/User" }
                                    }
                                }
                            }
                        }
                    }
                },
                "/users/{id}": {
                    "get": {
                        "operationId": "getUser",
                        "parameters": [
                            {
                                "name": "id",
                                "in": "path",
                                "required": true,
                                "schema": { "type": "integer" }
                            }
                        ],
                        "responses": {
                            "200": {
                                "description": "Success",
                                "content": {
                                    "application/json": {
                                        "schema": { "$ref": "#/components/schemas/User" }
                                    }
                                }
                            }
                        }
                    }
                }
            },
            "components": {
                "schemas": {
                    "User": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "integer" },
                            "name": { "type": "string" },
                            "email": { "type": "string", "format": "email" }
                        },
                        "required": ["id", "name"]
                    }
                }
            }
        });

        OpenApiSpec::from_json(&json).unwrap()
    }

    #[test]
    fn test_parse_spec() {
        let spec = sample_spec();
        assert_eq!(spec.openapi, "3.0.0");
        assert_eq!(spec.info.title, "Test API");
        assert!(spec.paths.contains_key("/users"));
    }

    #[test]
    fn test_get_operation() {
        let spec = sample_spec();
        let op = spec.get_operation("/users", "GET");
        assert!(op.is_some());
        assert_eq!(op.unwrap().operation_id, Some("getUsers".to_string()));
    }

    #[test]
    fn test_generate_response() {
        let spec = sample_spec();
        let mut rng = test_rng();

        let response = from_openapi(&mut rng, &spec, "/users", "GET", "200");
        assert!(response.is_some());

        let arr = response.unwrap();
        assert!(arr.is_array());
    }

    #[test]
    fn test_generate_single_user_response() {
        let spec = sample_spec();
        let mut rng = test_rng();

        let response = from_openapi(&mut rng, &spec, "/users/{id}", "GET", "200");
        assert!(response.is_some());

        let user = response.unwrap();
        assert!(user.is_object());
        assert!(user.get("id").is_some());
        assert!(user.get("name").is_some());
    }

    #[test]
    fn test_generate_request_body() {
        let spec = sample_spec();
        let mut rng = test_rng();

        let body = generate_request_body(&mut rng, &spec, "/users", "POST");
        assert!(body.is_some());

        let user = body.unwrap();
        assert!(user.is_object());
    }

    #[test]
    fn test_generate_parameters() {
        let spec = sample_spec();
        let mut rng = test_rng();

        let params = generate_parameters(&mut rng, &spec, "/users/{id}", "GET");
        assert!(params.contains_key("id"));
        assert!(params.get("id").unwrap().is_i64());
    }

    #[test]
    fn test_list_endpoints() {
        let spec = sample_spec();
        let endpoints = list_endpoints(&spec);

        assert!(endpoints.contains(&("/users".to_string(), "GET".to_string())));
        assert!(endpoints.contains(&("/users".to_string(), "POST".to_string())));
        assert!(endpoints.contains(&("/users/{id}".to_string(), "GET".to_string())));
    }

    #[test]
    fn test_example_override() {
        let json = json!({
            "openapi": "3.0.0",
            "info": { "title": "Test", "version": "1.0" },
            "paths": {
                "/test": {
                    "get": {
                        "responses": {
                            "200": {
                                "description": "OK",
                                "content": {
                                    "application/json": {
                                        "example": { "fixed": "value" }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        });

        let spec = OpenApiSpec::from_json(&json).unwrap();
        let mut rng = test_rng();

        let response = from_openapi(&mut rng, &spec, "/test", "GET", "200");
        assert_eq!(response, Some(json!({"fixed": "value"})));
    }
}
