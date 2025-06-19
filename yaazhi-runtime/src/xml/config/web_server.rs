use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct WebServer {
    pub host: ValueElement,
    #[serde(rename = "port-http")]
    pub port_http: ValueElement,
    #[serde(rename = "port-https")]
    pub port_https: ValueElement,
    #[serde(rename = "https-enabled")]
    pub https_enabled: ValueElement,
    #[serde(rename = "allow-origins")]
    pub allow_origins: ValueElement,
    #[serde(rename = "handle-cors")]
    pub handle_cors: ValueElement,
}

#[derive(Debug, Deserialize)]
pub struct ValueElement {
    #[serde(rename = "@value")]
    pub value: String,
}

impl WebServer {
    pub fn http_port(&self) -> Option<u16> {
        self.port_http.value.parse().ok()
    }

    pub fn https_port(&self) -> Option<u16> {
        self.port_https.value.parse().ok()
    }

    pub fn is_https_enabled(&self) -> bool {
        self.https_enabled.value.eq_ignore_ascii_case("true")
    }

    pub fn bind_address(&self) -> Option<String> {
        if self.is_https_enabled() {
            self.https_port()
                .map(|port| format!("{}:{}", self.host.value, port))
        } else {
            self.http_port()
                .map(|port| format!("{}:{}", self.host.value, port))
        }
    }

    pub fn allow_origins(&self) -> &str {
        &self.allow_origins.value
    }

    pub fn cors_enabled(&self) -> bool {
        self.handle_cors.value.eq_ignore_ascii_case("true")
    }
}
