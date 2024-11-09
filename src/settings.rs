use serde::Deserialize;
use toml::from_str;
use std::fs;
use anyhow::Context;

#[derive(Clone, Debug, Deserialize)]
pub struct Settings {
    pub host: String,
    pub port: u16,
}

impl Settings {
    pub fn new() -> Result<Settings, anyhow::Error> {
        let settings = fs::read_to_string("Settings.toml").context("Error reading Settings.toml")?;
        let settings = from_str(&settings).context("Error parsing Settings.toml")?;
        Ok(settings)
    }
}
