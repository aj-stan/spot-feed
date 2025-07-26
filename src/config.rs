use deadpool_postgres::{Config as PgConfig, Pool, Runtime};
use dotenv::dotenv;
use std::env;
use tokio_postgres::NoTls;

pub struct Config {
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub db_host: String,
    pub db_port: u16,
    pub bind_ip: String,
    pub bind_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();
        Self {
            db_name: env::var("DB_NAME").unwrap_or_else(|_| "spot_feed_db".to_string()),
            db_user: env::var("DB_USER").unwrap_or_else(|_| "spot_feed_user".to_string()),
            db_pass: env::var("DB_PASS").unwrap_or_else(|_| "secret".to_string()),
            db_host: env::var("DB_HOST").unwrap_or_else(|_| "localhost".to_string()),
            db_port: env::var("DB_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5432),
            bind_ip: env::var("BIND_IP").unwrap_or_else(|_| "127.0.0.1".to_string()),
            bind_port: env::var("BIND_PORT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(8080),
        }
    }

    pub fn make_pg_pool(&self) -> Pool {
        let mut cfg = PgConfig::new();
        cfg.dbname = Some(self.db_name.clone());
        cfg.user = Some(self.db_user.clone());
        cfg.password = Some(self.db_pass.clone());
        cfg.host = Some(self.db_host.clone());
        cfg.port = Some(self.db_port);
        cfg.create_pool(Some(Runtime::Tokio1), NoTls)
            .expect("Failed to create pool")
    }
}
