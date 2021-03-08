use clap::arg_enum;
use anyhow::{Context, Result};

arg_enum! {
    #[derive(Debug)] pub enum EntityType {
        Film,
        Actor,
    }
}

#[derive(Debug, serde::Deserialize)] pub struct Config {
    pub base_url: String,
}

pub fn get_config(
    config_path: &String
) -> Result<Config> {
    let contents = std::fs::read_to_string(&config_path)
        .with_context(|| format!("Could not read file: {}", &config_path))?;

    let config: Config = serde_json::from_str(&contents)
        .with_context(|| format!("Invalid JSON:\n {}", &contents))?;
    Ok(config)
}

