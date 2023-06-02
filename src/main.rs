mod sp; 
mod config;
pub mod routes {
    pub mod json_body;
    pub mod raw;
}

use axum::{
    http::{ StatusCode },
    routing::{ post },
    Router,
};

use serde::{Serialize};
// use serde_json::json;
use tracing_subscriber::{ layer::SubscriberExt, util::SubscriberInitExt };
use std::net::SocketAddr;



#[derive(Serialize)]
struct ResultVec {
    result: Vec<i32>,
}
#[tokio::main]
async fn main() {
    // let cfg = config::Config::new();
    let conn = sp::SteamPipe::new().await.unwrap();
    let pool = conn.get_pool().await;
    // println!("pool2: {:?}", pool2);

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rustypipe=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    // build our application with some routes
    let app = Router::new()

        .route(
            "/raw",
            post(routes::raw::raw_query),
        )
        .route(
            "/json_body",
            post(routes::json_body::json_body),
        )
        .with_state(pool);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

/// Utility function for mapping any error into a `500 Internal Server Error`
/// response.
fn internal_error<E>(err: E) -> (StatusCode, String)
where
    E: std::error::Error,
{
    (StatusCode::INTERNAL_SERVER_ERROR, err.to_string())
}
