use std::fs::File;
use std::io::BufReader;
use quick_xml::de::from_reader;

use super::entities::Entities;


pub fn load_entities(path: &str) -> Result<Entities, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let config: Entities = from_reader(reader)?;
    Ok(config)
}
