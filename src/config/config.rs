use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct AppConfig {
    pub if_login: bool,
    pub test: String,
    pub test2: u8,
}
