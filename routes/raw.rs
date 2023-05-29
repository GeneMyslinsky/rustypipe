use crate::sp::ConnectionPool;
use crate::{internal_error, ResultVec};
use axum::{
    extract::{State},
    http::{StatusCode},
};


pub async fn raw_query(
    State(pool): State<ConnectionPool>,
    query: String<>,
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