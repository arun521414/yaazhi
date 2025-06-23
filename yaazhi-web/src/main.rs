use yaazhi_core::xml::config::yaazhi_config::{self, load_config, YaazhiConfig};
use yaazhi_core::db::{
    connection::Connection,
    sqlite::{SqliteConfig,check_pool_connection}
};
use std::error::Error;

mod server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {


    tracing_subscriber::fmt::init();

    let config_path = "/home/arun/yaazhi-projects/yaazhi/yaazhi-runtime/config/YaazhiConfig.xml";
    let config: YaazhiConfig = load_config(config_path)?;

    // Initialize the database connection pool
    let sqlite_config = SqliteConfig::default();

    let db_pool = Connection::new_sqlite_pool(sqlite_config).await?; 

     check_pool_connection(&db_pool).await?;

    server::start_server(config).await
}
