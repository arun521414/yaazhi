use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub locale: ValueElement,
    pub time_zone: ValueElement,
    #[serde(rename = "database_time_zone")]
    pub database_time_zone: ValueElement,
}

#[derive(Debug, Deserialize)]
pub struct ValueElement {
    #[serde(rename = "@value")]
    pub value: String,
}

impl Settings {
    pub fn locale(&self) -> &str {
        &self.locale.value
    }

    pub fn time_zone(&self) -> &str {
        &self.time_zone.value
    }

    pub fn database_time_zone(&self) -> &str {
        &self.database_time_zone.value
    }

    pub fn is_same_time_zone(&self) -> bool {
        self.time_zone.value == self.database_time_zone.value
    }
}