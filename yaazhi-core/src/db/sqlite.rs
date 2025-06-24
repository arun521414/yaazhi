// yaazhi-core/src/db/sqlite.rs

use std::{path::PathBuf, time::Duration};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};
use tracing::info;

#[derive(Debug, Clone)]
pub struct SqliteConfig {
    pub file_path: String,
    pub create_file: bool,
    pub read_only: bool,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub acquire_timeout: u64,
    pub idle_timeout: u64,
    pub logging_enabled: bool,
    pub logging_level: String,
}

impl Default for SqliteConfig {
    fn default() -> Self {
        Self {
            file_path: default_sqlite_db_path().to_string_lossy().into_owned(),
            create_file: true,
            read_only: false,
            max_connections: 10,
            min_connections: 5,
            connect_timeout: 5,
            acquire_timeout: 5,
            idle_timeout: 10,
            logging_enabled: false,
            logging_level: "info".to_string(),
        }
    }
}

/// Returns the path to the workspace root
fn workspace_root() -> PathBuf {
    // Get the directory of the current crate's Cargo.toml
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    // Go up one level to reach the workspace root
    manifest_dir.parent().expect("Failed to find workspace root").to_path_buf()
}

/// Returns the path to the yaazhi-runtime folder
fn default_runtime_dir() -> PathBuf {
    workspace_root().join("yaazhi-runtime")
}

/// Returns the default SQLite DB path
fn default_sqlite_db_path() -> PathBuf {
    default_runtime_dir()
        .join("db")
        .join("sqlite")
        .join("yaazhi.db")
}

/// Ensures parent directories exist for the DB file
fn ensure_db_dir_exists(db_path: &str) -> Result<(), std::io::Error> {
    let binding = PathBuf::from(db_path);
    let db_dir = binding
        .parent()
        .expect("Database path has no parent directory");

    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir)?;
    }

    Ok(())
}

/// Initialize SQLite connection pool
pub async fn init_sqlite_pool(config: SqliteConfig) -> Result<SqlitePool, sqlx::Error> {
    
    ensure_db_dir_exists(&config.file_path)?;

    let options = SqliteConnectOptions::new()
        .filename(&config.file_path)
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(config.create_file)
        .read_only(config.read_only)
        .busy_timeout(Duration::from_secs(config.connect_timeout));

    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .connect_with(options)
        .await?;

    Ok(pool)
}

pub async fn check_pool_connection(pool: &SqlitePool) -> Result<(), sqlx::Error> {
    let row: (i64,) = sqlx::query_as("SELECT 1")
        .fetch_one(pool)
        .await?;

    assert_eq!(row.0, 1);
    info!("✅ Database connection successful!");

    Ok(())
}
