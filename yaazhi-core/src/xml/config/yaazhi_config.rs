use serde::Deserialize;
use super::{settings::Settings,web_server::WebServer, databases::Databases};


#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct YaazhiConfig {
    pub settings : Settings,
    pub web_server: WebServer,
    pub databases: Databases,
}



use std::fs::File;
use std::io::BufReader;
use quick_xml::de::from_reader;


pub fn load_config(path: &str) -> Result<YaazhiConfig, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: YaazhiConfig = from_reader(reader)?;
    Ok(config)
}