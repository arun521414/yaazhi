use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Databases {
    #[serde(rename = "data-types")]
    pub data_types: DataTypes,
    #[serde(rename = "connection")]
    pub connections: Vec<Connection>,
    #[serde(rename = "default")]
    pub default_db: DefaultDb,
}

#[derive(Debug, Deserialize)]
pub struct DataTypes {
    #[serde(rename = "property")]
    pub properties: Vec<DataTypeProperty>,
}

#[derive(Debug, Deserialize)]
pub struct DataTypeProperty {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@pg-sql-type")]
    pub pg_sql_type: String,
    #[serde(rename = "@mysql-sql-type")]
    pub mysql_sql_type: String,
    #[serde(rename = "@sqlite-sql-type")]
    pub sqlite_sql_type: String,
}

#[derive(Debug, Deserialize)]
pub struct Connection {
    #[serde(rename = "@name")]
    pub name: String,
    #[serde(rename = "@type")]
    pub conn_type: String,

    #[serde(rename = "file-path")]
    pub file_path: Option<ValueElement>,
    #[serde(rename = "create-file")]
    pub create_file: Option<ValueElement>,
    #[serde(rename = "read-only")]
    pub read_only: Option<ValueElement>,

    pub host: Option<ValueElement>,
    pub port: Option<ValueElement>,
    pub username: Option<ValueElement>,
    pub password: Option<ValueElement>,
    pub database: Option<ValueElement>,

    pub pool: Option<ConnectionPool>,
}

#[derive(Debug, Deserialize)]
pub struct ConnectionPool {
    #[serde(rename = "@max")]
    pub max: u32,
    #[serde(rename = "@min")]
    pub min: u32,

    pub timeouts: Option<Timeouts>,
    pub logging: Option<Logging>,
}

#[derive(Debug, Deserialize)]
pub struct Timeouts {
    #[serde(rename = "@connect")]
    pub connect: u32,
    #[serde(rename = "@acquire")]
    pub acquire: u32,
    #[serde(rename = "@idle")]
    pub idle: u32,
}

#[derive(Debug, Deserialize)]
pub struct Logging {
    #[serde(rename = "@enabled")]
    pub enabled: bool,
    #[serde(rename = "@level")]
    pub level: String,
}

#[derive(Debug, Deserialize)]
pub struct DefaultDb {
    #[serde(rename = "@name")]
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct ValueElement {
    #[serde(rename = "@value")]
    pub value: String,
}


impl Databases {
    /// Get the default connection if present by matching the name
    pub fn get_default_connection(&self) -> Option<&Connection> {
        self.connections.iter().find(|c| c.name == self.default_db.name)
    }

    /// Get a connection by name
    pub fn get_connection(&self, name: &str) -> Option<&Connection> {
        self.connections.iter().find(|c| c.name == name)
    }

    /// List all connection names
    pub fn list_connection_names(&self) -> Vec<&str> {
        self.connections.iter().map(|c| c.name.as_str()).collect()
    }
}

impl Connection {

    pub fn get_param(&self, key: &str) -> Option<&str> {
        match key {
            "file-path" => self.file_path.as_ref().map(|v| v.value.as_str()),
            "create-file" => self.create_file.as_ref().map(|v| v.value.as_str()),
            "read-only" => self.read_only.as_ref().map(|v| v.value.as_str()),
            "host" => self.host.as_ref().map(|v| v.value.as_str()),
            "port" => self.port.as_ref().map(|v| v.value.as_str()),
            "username" => self.username.as_ref().map(|v| v.value.as_str()),
            "password" => self.password.as_ref().map(|v| v.value.as_str()),
            "database" => self.database.as_ref().map(|v| v.value.as_str()),
            _ => None,
        }
    }


    pub fn is_file_based(&self) -> bool {
        self.conn_type.eq_ignore_ascii_case("sqlite") || self.file_path.is_some()
    }


    pub fn masked_connection_string(&self) -> String {
        format!(
            "{}://{}:{}@{}:{}/{}",
            self.conn_type,
            self.username
                .as_ref()
                .map(|v| v.value.as_str())
                .unwrap_or("user"),
            if self.password.is_some() { "****" } else { "" },
            self.host.as_ref().map(|v| v.value.as_str()).unwrap_or("localhost"),
            self.port.as_ref().map(|v| v.value.as_str()).unwrap_or(""),
            self.database
                .as_ref()
                .map(|v| v.value.as_str())
                .unwrap_or("")
        )
    }
}


impl ConnectionPool {
    /// Checks if the pool settings are valid
    pub fn is_valid(&self) -> bool {
        self.min <= self.max
    }

    /// Returns the total pool size range as a tuple
    pub fn range(&self) -> (u32, u32) {
        (self.min, self.max)
    }
}


impl Logging {

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    pub fn level(&self) -> &str {
        &self.level
    }

}


impl ValueElement {

    pub fn as_str(&self) -> &str {
        &self.value
    }

    pub fn parse_as<T: std::str::FromStr>(&self) -> Option<T> {
        self.value.parse::<T>().ok()
    }

}