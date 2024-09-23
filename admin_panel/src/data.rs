use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::{self, Read, Write};

#[derive(Clone)]
pub struct MetaData {
    pub title: String,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct ServerSettings {
    pub address: String,
    pub api_token: String,
}

impl Default for ServerSettings {
    fn default() -> Self {
        Self {
            address: String::from(""),
            api_token: String::from(""),
        }
    }
}

pub fn load_server_settings() -> ServerSettings {
    if let Ok(mut file) = File::open("mdap_serversettings.json") {
        let mut content = String::new();
        if file.read_to_string(&mut content).is_ok() {
            if let Ok(state) = serde_json::from_str(&content) {
                return state;
            }
        }
    }
    ServerSettings::default()
}

pub fn save_server_settings(state: &ServerSettings) -> io::Result<()> {
    let content = serde_json::to_string(state)?;
    let mut file = File::create("mdap_serversettings.json")?;
    file.write_all(content.as_bytes())
}
