use std::{env, string};
use dotenv::dotenv;



pub struct Config {
    pub db_host: String,
    pub db_user: String,
    pub db_port: String,
    pub db_password: String,
}

impl Config {
    pub fn new() -> Self {
        dotenv().ok();

        Self {
            db_host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_user: env::var("DB_USER").unwrap_or_else(|_| "steampipe".to_string()),
            db_port: env::var("DB_PORT").unwrap_or_else(|_| "9193".to_string()),
            db_password: env::var("DB_PASSWORD").unwrap_or_else(|_| "".to_string()),
        }
    }

    pub fn get_sp_connstring(&self) -> String<> {
        format!(
            "host={} user={} port={} password={}",
            self.db_host, self.db_user, self.db_port, self.db_password
        )
        // PostgresConnectionManager::new_from_stringlike(conn_string, NoTls).unwrap()
    
    }
}