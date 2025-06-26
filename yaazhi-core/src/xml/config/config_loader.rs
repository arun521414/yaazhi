use std::fs::File;
use std::io::BufReader;
use quick_xml::de::from_reader;

use super::yaazhi_config::YaazhiConfig;


pub fn load_config(path: &str) -> Result<YaazhiConfig, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: YaazhiConfig = from_reader(reader)?;
    Ok(config)
}