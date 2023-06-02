use indexmap::IndexMap;
// use tokio_postgres::types::;
use serde_json::{Value as JsonValue, json};
use tokio_postgres::types::{Type, Json};
use serde::{Serialize};
use crate::sp::ConnectionPool;
use crate::{internal_error};
use axum::{
    extract::{State},
    http::{StatusCode},
};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)] // add this line
pub enum PgValue {
    Bool(bool),
    String(String),
    Int(i32),
    Float(f64),
    JsonV(JsonValue),
    // and so on for other data types...
}


pub async fn raw_query(
    State(pool): State<ConnectionPool>,
    query: String,
) -> Result<String, (StatusCode, String)> {
    let conn = pool.get().await.map_err(internal_error)?;

    let rows = conn
        .query(query.as_str(), &[])
        .await
        .map_err(internal_error)?;

    let mut results = vec![];
    for row in &rows {
        let mut row_map = IndexMap::new();  
        for (i, column) in row.columns().iter().enumerate() {
            let value = match column.type_() {
                &Type::BOOL => PgValue::Bool(row.get(i)),
                &Type::TEXT => PgValue::String(row.get(i)),
                &Type::INT4 => PgValue::Int(row.get(i)),
                &Type::FLOAT4 | &Type::FLOAT8 => PgValue::Float(row.get(i)),
                &Type::JSONB => {
                    let value_result: Result<serde_json::Value, _> = row.try_get(i);
                    match value_result {
                        Ok(value) => PgValue::JsonV(value),
                        Err(_) => PgValue::JsonV(json!(null)), // if the value is NULL or can't be converted to serde_json::Value
                    }
                },
                _ => PgValue::String(row.try_get(i).unwrap_or_else(|_| "".to_string())), 
            };
            row_map.insert(column.name().to_string(), value);
        }
        results.push(row_map);
    }

    Ok(serde_json::to_string(&results).map_err(internal_error)?)
}