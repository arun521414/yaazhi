use yaazhi_runtime::xml::config::yaazhi_config::{YaazhiConfig, load_config};
use std::error::Error;

mod server;

#[actix_web::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let config_path = "/home/arun/yaazhi-projects/yaazhi/yaazhi-runtime/config/YaazhiConfig.xml";
    let config: YaazhiConfig = load_config(config_path)?;

    server::start_server(config).await
}
