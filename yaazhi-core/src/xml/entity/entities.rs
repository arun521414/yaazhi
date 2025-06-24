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
    #[serde(rename = "field")]
    pub fields: Vec<Field>,
}

#[derive(Debug, Deserialize)]
pub struct Field {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub field_type: String,
    #[serde(rename = "@pk")]
    pub primary_key: bool,
    #[serde(rename = "@unique")]
    pub unique: bool,
    #[serde(rename = "@nullable")]
    pub null: bool,
    #[serde(rename = "@default")]
    pub default: Option<String>,
    #[serde(rename = "@index")]
    pub index: bool,
    #[serde(rename = "@foreign-key")]
    pub foreign_key: Option<String>,
    #[serde(rename = "@foreign-key-on-delete")]
    pub foreign_key_on_delete: Option<String>,
    #[serde(rename = "@foreign-key-on-update")]
    pub foreign_key_on_update: Option<String>,
    #[serde(rename = "description")]
    pub description: Option<String>,
}