//! SQL DDL and DML generation.
//!
//! Generate CREATE TABLE statements and INSERT statements from data.

use serde::Serialize;
use serde_json::{Map, Value};

/// SQL dialect for generating statements.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum SqlDialect {
    /// PostgreSQL dialect
    #[default]
    PostgreSQL,
    /// MySQL dialect
    MySQL,
    /// SQLite dialect
    SQLite,
    /// Microsoft SQL Server dialect
    SqlServer,
}

impl SqlDialect {
    /// Get the string quote character for this dialect.
    fn string_quote(&self) -> char {
        match self {
            SqlDialect::MySQL => '"',
            _ => '\'',
        }
    }

    /// Get the identifier quote character for this dialect.
    fn identifier_quote(&self) -> &str {
        match self {
            SqlDialect::MySQL => "`",
            SqlDialect::SqlServer => "\"",
            _ => "\"",
        }
    }

    /// Get NULL keyword.
    fn null_keyword(&self) -> &str {
        "NULL"
    }

    /// Get boolean true value.
    fn bool_true(&self) -> &str {
        match self {
            SqlDialect::PostgreSQL => "TRUE",
            SqlDialect::MySQL => "1",
            SqlDialect::SQLite => "1",
            SqlDialect::SqlServer => "1",
        }
    }

    /// Get boolean false value.
    fn bool_false(&self) -> &str {
        match self {
            SqlDialect::PostgreSQL => "FALSE",
            SqlDialect::MySQL => "0",
            SqlDialect::SQLite => "0",
            SqlDialect::SqlServer => "0",
        }
    }
}

/// Generate a CREATE TABLE statement from a JSON Schema.
///
/// # Arguments
///
/// * `table_name` - Name of the table to create
/// * `schema` - JSON Schema describing the table structure
///
/// # Returns
///
/// A CREATE TABLE SQL statement.
///
/// # Example
///
/// ```
/// use dx_datagen::schema::to_sql_ddl;
/// use serde_json::json;
///
/// let schema = json!({
///     "type": "object",
///     "properties": {
///         "id": { "type": "integer" },
///         "name": { "type": "string", "maxLength": 100 },
///         "email": { "type": "string", "format": "email" },
///         "active": { "type": "boolean" }
///     },
///     "required": ["id", "name"]
/// });
///
/// let ddl = to_sql_ddl("users", &schema, Default::default());
/// assert!(ddl.contains("CREATE TABLE"));
/// assert!(ddl.contains("users"));
/// ```
pub fn to_sql_ddl(table_name: &str, schema: &Value, dialect: SqlDialect) -> String {
    let iq = dialect.identifier_quote();
    let mut columns = Vec::new();

    let obj = match schema.as_object() {
        Some(o) => o,
        None => return format!("CREATE TABLE {iq}{table_name}{iq} ();"),
    };

    let properties = match obj.get("properties").and_then(|p| p.as_object()) {
        Some(p) => p,
        None => return format!("CREATE TABLE {iq}{table_name}{iq} ();"),
    };

    let required: std::collections::HashSet<String> = obj
        .get("required")
        .and_then(|r| r.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect()
        })
        .unwrap_or_default();

    for (col_name, col_schema) in properties {
        let col_obj = match col_schema.as_object() {
            Some(o) => o,
            None => continue,
        };

        let sql_type = json_type_to_sql(col_obj, dialect);
        let nullable = if required.contains(col_name) {
            " NOT NULL"
        } else {
            ""
        };

        columns.push(format!("    {iq}{col_name}{iq} {sql_type}{nullable}"));
    }

    if columns.is_empty() {
        format!("CREATE TABLE {iq}{table_name}{iq} ();")
    } else {
        format!(
            "CREATE TABLE {iq}{table_name}{iq} (\n{}\n);",
            columns.join(",\n")
        )
    }
}

/// Convert JSON Schema type to SQL type.
fn json_type_to_sql(schema: &Map<String, Value>, dialect: SqlDialect) -> String {
    let type_val = schema
        .get("type")
        .and_then(|t| t.as_str())
        .unwrap_or("string");

    match type_val {
        "integer" => {
            let min = schema.get("minimum").and_then(|v| v.as_i64());
            let max = schema.get("maximum").and_then(|v| v.as_i64());

            match (min, max, dialect) {
                (Some(min), Some(max), _) if min >= 0 && max <= 255 => "SMALLINT".to_string(),
                (Some(min), Some(max), _) if min >= -32768 && max <= 32767 => {
                    "SMALLINT".to_string()
                }
                (Some(min), Some(max), _) if min >= -2147483648 && max <= 2147483647 => {
                    "INTEGER".to_string()
                }
                _ => "BIGINT".to_string(),
            }
        }
        "number" => match dialect {
            SqlDialect::PostgreSQL => "DOUBLE PRECISION".to_string(),
            SqlDialect::MySQL => "DOUBLE".to_string(),
            SqlDialect::SQLite => "REAL".to_string(),
            SqlDialect::SqlServer => "FLOAT".to_string(),
        },
        "boolean" => match dialect {
            SqlDialect::PostgreSQL => "BOOLEAN".to_string(),
            SqlDialect::MySQL => "TINYINT(1)".to_string(),
            SqlDialect::SQLite => "INTEGER".to_string(),
            SqlDialect::SqlServer => "BIT".to_string(),
        },
        "string" => {
            // Check format first
            if let Some(format) = schema.get("format").and_then(|f| f.as_str()) {
                return match (format, dialect) {
                    ("date", _) => "DATE".to_string(),
                    ("time", _) => "TIME".to_string(),
                    ("date-time", SqlDialect::PostgreSQL) => "TIMESTAMP WITH TIME ZONE".to_string(),
                    ("date-time", SqlDialect::MySQL) => "DATETIME".to_string(),
                    ("date-time", _) => "TIMESTAMP".to_string(),
                    ("uuid", SqlDialect::PostgreSQL) => "UUID".to_string(),
                    ("uuid", _) => "CHAR(36)".to_string(),
                    ("email", _) => "VARCHAR(255)".to_string(),
                    ("uri", _) | ("url", _) => "VARCHAR(2048)".to_string(),
                    ("ipv4", _) => "VARCHAR(15)".to_string(),
                    ("ipv6", _) => "VARCHAR(45)".to_string(),
                    _ => "VARCHAR(255)".to_string(),
                };
            }

            // Check maxLength
            if let Some(max_len) = schema.get("maxLength").and_then(|m| m.as_u64()) {
                if max_len <= 255 {
                    return format!("VARCHAR({})", max_len);
                } else if max_len <= 65535 {
                    return match dialect {
                        SqlDialect::PostgreSQL => "TEXT".to_string(),
                        SqlDialect::MySQL => "TEXT".to_string(),
                        SqlDialect::SQLite => "TEXT".to_string(),
                        SqlDialect::SqlServer => format!("VARCHAR({})", max_len.min(8000)),
                    };
                }
            }

            // Default string type
            match dialect {
                SqlDialect::PostgreSQL => "TEXT".to_string(),
                SqlDialect::MySQL => "TEXT".to_string(),
                SqlDialect::SQLite => "TEXT".to_string(),
                SqlDialect::SqlServer => "NVARCHAR(MAX)".to_string(),
            }
        }
        "array" => {
            // JSON array type
            match dialect {
                SqlDialect::PostgreSQL => "JSONB".to_string(),
                SqlDialect::MySQL => "JSON".to_string(),
                SqlDialect::SQLite => "TEXT".to_string(),
                SqlDialect::SqlServer => "NVARCHAR(MAX)".to_string(),
            }
        }
        "object" => {
            // JSON object type
            match dialect {
                SqlDialect::PostgreSQL => "JSONB".to_string(),
                SqlDialect::MySQL => "JSON".to_string(),
                SqlDialect::SQLite => "TEXT".to_string(),
                SqlDialect::SqlServer => "NVARCHAR(MAX)".to_string(),
            }
        }
        _ => "TEXT".to_string(),
    }
}

/// Generate an INSERT statement from serializable records.
///
/// # Arguments
///
/// * `table_name` - Name of the table to insert into
/// * `records` - Slice of serializable records
///
/// # Returns
///
/// An INSERT SQL statement.
pub fn to_sql_insert<T: Serialize>(table_name: &str, records: &[T], dialect: SqlDialect) -> String {
    let values: Vec<Value> = records
        .iter()
        .filter_map(|r| serde_json::to_value(r).ok())
        .collect();

    to_sql_insert_values(table_name, &values, dialect)
}

/// Generate an INSERT statement from JSON values.
fn to_sql_insert_values(table_name: &str, values: &[Value], dialect: SqlDialect) -> String {
    if values.is_empty() {
        return String::new();
    }

    let iq = dialect.identifier_quote();

    // Get columns from first record
    let first_obj = match values.first().and_then(|v| v.as_object()) {
        Some(o) => o,
        None => return String::new(),
    };

    let columns: Vec<&String> = first_obj.keys().collect();
    if columns.is_empty() {
        return String::new();
    }

    let column_list = columns
        .iter()
        .map(|c| format!("{iq}{c}{iq}"))
        .collect::<Vec<_>>()
        .join(", ");

    let mut rows = Vec::new();
    for value in values {
        if let Some(obj) = value.as_object() {
            let row_values: Vec<String> = columns
                .iter()
                .map(|col| {
                    obj.get(*col)
                        .map(|v| value_to_sql(v, dialect))
                        .unwrap_or_else(|| dialect.null_keyword().to_string())
                })
                .collect();
            rows.push(format!("({})", row_values.join(", ")));
        }
    }

    format!(
        "INSERT INTO {iq}{table_name}{iq} ({column_list}) VALUES\n{};",
        rows.join(",\n")
    )
}

/// Generate batch INSERT statements.
///
/// # Arguments
///
/// * `table_name` - Name of the table to insert into
/// * `records` - Slice of serializable records
/// * `batch_size` - Number of records per INSERT statement
///
/// # Returns
///
/// A vector of INSERT SQL statements.
pub fn to_sql_insert_batch<T: Serialize>(
    table_name: &str,
    records: &[T],
    batch_size: usize,
    dialect: SqlDialect,
) -> Vec<String> {
    let values: Vec<Value> = records
        .iter()
        .filter_map(|r| serde_json::to_value(r).ok())
        .collect();

    values
        .chunks(batch_size)
        .map(|chunk| to_sql_insert_values(table_name, chunk, dialect))
        .filter(|s| !s.is_empty())
        .collect()
}

/// Convert a JSON value to SQL literal.
fn value_to_sql(value: &Value, dialect: SqlDialect) -> String {
    match value {
        Value::Null => dialect.null_keyword().to_string(),
        Value::Bool(b) => {
            if *b {
                dialect.bool_true().to_string()
            } else {
                dialect.bool_false().to_string()
            }
        }
        Value::Number(n) => n.to_string(),
        Value::String(s) => {
            let escaped = s.replace('\'', "''");
            let q = dialect.string_quote();
            format!("{q}{escaped}{q}")
        }
        Value::Array(_) | Value::Object(_) => {
            // Serialize as JSON string
            let json_str = serde_json::to_string(value).unwrap_or_default();
            let escaped = json_str.replace('\'', "''");
            let q = dialect.string_quote();
            format!("{q}{escaped}{q}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Serialize;
    use serde_json::json;

    #[test]
    fn test_ddl_basic() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "name": { "type": "string" }
            },
            "required": ["id"]
        });

        let ddl = to_sql_ddl("users", &schema, SqlDialect::PostgreSQL);
        assert!(ddl.contains("CREATE TABLE"));
        assert!(ddl.contains("users"));
        assert!(ddl.contains("id"));
        assert!(ddl.contains("name"));
        assert!(ddl.contains("NOT NULL"));
    }

    #[test]
    fn test_ddl_types() {
        let schema = json!({
            "type": "object",
            "properties": {
                "id": { "type": "integer" },
                "price": { "type": "number" },
                "active": { "type": "boolean" },
                "email": { "type": "string", "format": "email" },
                "created": { "type": "string", "format": "date-time" }
            }
        });

        let ddl = to_sql_ddl("products", &schema, SqlDialect::PostgreSQL);
        assert!(ddl.contains("BIGINT") || ddl.contains("INTEGER"));
        assert!(ddl.contains("DOUBLE PRECISION"));
        assert!(ddl.contains("BOOLEAN"));
        assert!(ddl.contains("VARCHAR(255)"));
        assert!(ddl.contains("TIMESTAMP"));
    }

    #[test]
    fn test_ddl_mysql() {
        let schema = json!({
            "type": "object",
            "properties": {
                "active": { "type": "boolean" },
                "data": { "type": "object" }
            }
        });

        let ddl = to_sql_ddl("items", &schema, SqlDialect::MySQL);
        assert!(ddl.contains("TINYINT(1)"));
        assert!(ddl.contains("JSON"));
    }

    #[derive(Serialize)]
    struct TestRecord {
        id: i32,
        name: String,
        active: bool,
    }

    #[test]
    fn test_insert_basic() {
        let records = vec![
            TestRecord {
                id: 1,
                name: "Alice".into(),
                active: true,
            },
            TestRecord {
                id: 2,
                name: "Bob".into(),
                active: false,
            },
        ];

        let sql = to_sql_insert("users", &records, SqlDialect::PostgreSQL);
        assert!(sql.contains("INSERT INTO"));
        assert!(sql.contains("users"));
        assert!(sql.contains("Alice"));
        assert!(sql.contains("Bob"));
        assert!(sql.contains("TRUE"));
        assert!(sql.contains("FALSE"));
    }

    #[test]
    fn test_insert_escaping() {
        let records = vec![TestRecord {
            id: 1,
            name: "O'Brien".into(),
            active: true,
        }];

        let sql = to_sql_insert("users", &records, SqlDialect::PostgreSQL);
        assert!(sql.contains("O''Brien")); // Escaped single quote
    }

    #[test]
    fn test_insert_batch() {
        let records = vec![
            TestRecord {
                id: 1,
                name: "A".into(),
                active: true,
            },
            TestRecord {
                id: 2,
                name: "B".into(),
                active: true,
            },
            TestRecord {
                id: 3,
                name: "C".into(),
                active: true,
            },
        ];

        let batches = to_sql_insert_batch("users", &records, 2, SqlDialect::PostgreSQL);
        assert_eq!(batches.len(), 2);
        assert!(batches[0].contains("A"));
        assert!(batches[0].contains("B"));
        assert!(batches[1].contains("C"));
    }

    #[test]
    fn test_dialect_mysql() {
        let records = vec![TestRecord {
            id: 1,
            name: "Test".into(),
            active: true,
        }];

        let sql = to_sql_insert("users", &records, SqlDialect::MySQL);
        // MySQL uses 1 for true
        assert!(sql.contains("1"));
        // MySQL uses backticks
        assert!(sql.contains("`"));
    }
}
