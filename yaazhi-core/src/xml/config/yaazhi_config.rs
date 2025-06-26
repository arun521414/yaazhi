use serde::Deserialize;
use super::{settings::Settings,web_server::WebServer, databases::Databases};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct YaazhiConfig {
    pub settings : Settings,
    pub web_server: WebServer,
    pub databases: Databases,
}



