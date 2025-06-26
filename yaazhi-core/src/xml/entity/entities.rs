use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Entities {
    #[serde(rename = "entity")]
    pub entities: Vec<Entity>,
}

#[derive(Debug, Deserialize)]
pub struct Entity {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@module")]
    pub module: String,
    #[serde(rename = "@cache")]
    pub cache: Option<bool>,
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub field_type: String,
    #[serde(rename = "@is-pk")]
    pub primary_key: Option<bool>,
}


