//! Schema-based data generation module.
//!
//! This module provides functionality for generating data from various schema formats
//! and inferring schemas from data.
//!
//! # Features
//!
//! - **JSON Schema**: Generate random data conforming to JSON Schema definitions
//! - **Schema Inference**: Infer JSON Schema from data samples
//! - **SQL DDL**: Generate CREATE TABLE and INSERT statements
//! - **OpenAPI**: Generate mock API responses from OpenAPI specs
//! - **Avro**: Generate data from Avro schemas (feature-gated)
//! - **GraphQL**: Generate mock data from GraphQL type definitions
//!
//! # Example
//!
//! ```
//! use dx_datagen::schema::{from_json_schema, infer_schema};
//! use rand::SeedableRng;
//! use rand::rngs::StdRng;
//! use serde_json::json;
//!
//! let mut rng = StdRng::seed_from_u64(42);
//!
//! // Generate data from JSON Schema
//! let schema = json!({
//!     "type": "object",
//!     "properties": {
//!         "name": { "type": "string" },
//!         "age": { "type": "integer", "minimum": 0, "maximum": 120 }
//!     },
//!     "required": ["name", "age"]
//! });
//!
//! let data = from_json_schema(&mut rng, &schema);
//! ```

mod avro;
mod graphql;
mod inference;
mod json_schema;
mod openapi;
mod sql;

// Re-export main functions
pub use avro::{from_avro_schema, to_avro_schema};
pub use graphql::{from_graphql_query, from_graphql_schema};
pub use inference::{infer_schema, infer_schema_from_values};
pub use json_schema::{from_json_schema, from_json_schema_with_options, JsonSchemaOptions};
pub use openapi::{from_openapi, OpenApiSpec};
pub use sql::{to_sql_ddl, to_sql_insert, to_sql_insert_batch, SqlDialect};
