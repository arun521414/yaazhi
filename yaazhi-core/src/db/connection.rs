use sqlx::{AnyPool, MySqlPool, PgPool, SqlitePool};

use crate::db::sqlite::{init_sqlite_pool, SqliteConfig};

pub struct Connection;

impl Connection {

    pub async fn new_sqlite_pool(config: SqliteConfig) -> Result<SqlitePool, sqlx::Error> {
        let pool = init_sqlite_pool(config).await?;
        Ok(pool)
    }
    pub async fn new_mysql_pool() -> Result<MySqlPool, sqlx::Error> {
        // Placeholder for MySQL pool initialization
        unimplemented!()
    }

    pub async fn new_postgresql_pool() -> Result<PgPool, sqlx::Error> {
        // Placeholder for PostgreSQL pool initialization
        unimplemented!()
    }
    
}