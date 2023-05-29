use crate::config;
use tokio_postgres::NoTls;
use bb8::{Pool};
use bb8_postgres::PostgresConnectionManager;
use std::sync::Arc;

pub struct SteamPipe {
    pub pool: Arc<Pool<PostgresConnectionManager<NoTls>>>,
}
pub type ConnectionPool = Arc<Pool<PostgresConnectionManager<NoTls>>>;

impl SteamPipe {
    pub async fn new() -> Result<Self, bb8::RunError<bb8_postgres::tokio_postgres::Error>> {
        let config = config::Config::new();
        let manager = PostgresConnectionManager::new_from_stringlike(config.get_sp_connstring().as_str(), NoTls).unwrap();
        let pool = Pool::builder().build(manager).await?;
        Ok(Self {
            pool: Arc::new(pool),
        })
    }

    pub async fn get_pool(&self) -> Arc<Pool<PostgresConnectionManager<NoTls>>> {
        Arc::clone(&self.pool)
    }
}
