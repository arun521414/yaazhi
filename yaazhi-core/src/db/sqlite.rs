use std::{path::PathBuf, time::Duration};

use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqlitePoolOptions};

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

// --- Path helpers ---

fn default_runtime_dir() -> PathBuf {
    PathBuf::from("yaazhi-runtime")
}

fn default_sqlite_db_path() -> PathBuf {
    default_runtime_dir()
        .join("db")
        .join("sqlite")
        .join("yaazhi.db")
}

fn resolve_path(path: &str) -> PathBuf {
    let mut resolved = std::env::current_dir().unwrap_or_default();
    resolved.push(path);
    resolved
}

fn ensure_db_dir_exists(db_path: &str) -> Result<(), std::io::Error> {
    let db_dir = std::path::Path::new(db_path)
        .parent()
        .expect("Database path has no parent directory");

    if !db_dir.exists() {
        std::fs::create_dir_all(db_dir)?;
    }

    Ok(())
}

// --- Pool Init ---

pub async fn init_sqlite_pool(config: SqliteConfig) -> Result<SqlitePool, sqlx::Error> {
    // Ensure parent directories exist
    ensure_db_dir_exists(&config.file_path)?;

    // Resolve full path
    let resolved_path = resolve_path(&config.file_path);

    // Build connection options
    let options = SqliteConnectOptions::new()
        .filename(resolved_path.as_os_str()) // <-- safe path here
        .journal_mode(SqliteJournalMode::Wal)
        .create_if_missing(config.create_file)
        .read_only(config.read_only)
        .busy_timeout(Duration::from_secs(config.connect_timeout));

    // Build pool
    let pool = SqlitePoolOptions::new()
        .max_connections(config.max_connections)
        .min_connections(config.min_connections)
        .acquire_timeout(Duration::from_secs(config.acquire_timeout))
        .idle_timeout(Duration::from_secs(config.idle_timeout))
        .connect_with(options)
        .await?;

    Ok(pool)
}