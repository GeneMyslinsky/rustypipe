use crate::sp::ConnectionPool;
use crate::{internal_error, ResultVec};
use axum::{
    extract::{State, Json},
    http::{StatusCode},
};
use serde::{Deserialize};

#[derive(Debug, Deserialize)]
pub struct Query {
    query: String,
}

pub async fn json_body(
    State(pool): State<ConnectionPool>,
    Json(Query { query }): Json<Query>,
) -> Result<String, (StatusCode, String)> {
    // rest of the code
    let conn = pool.get().await.map_err(internal_error)?;

    let rows = conn
        .query(query.as_str(), &[])  // replace hard-coded query with the one from the request body
        .await
        .map_err(internal_error)?;

    let mut results = vec![];

    for row in rows {
        let res: i32 = row.try_get(0).map_err(internal_error)?;
        results.push(res);
    }

    let response = ResultVec { result: results };

    Ok(serde_json::to_string(&response).map_err(internal_error)?)
}
