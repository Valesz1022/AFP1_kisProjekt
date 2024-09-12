use std::{env, io};

use config::{Config, ConfigError};
use serde::Deserialize;
use serde_aux::field_attributes;
use thiserror::Error;

#[non_exhaustive]
#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to determine the current working directory.")]
    Cwd(#[from] io::Error),
    #[error("Failed to read configuration.")]
    Config(#[from] ConfigError),
}

#[non_exhaustive]
#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    pub host: String,
    #[serde(
        deserialize_with = "field_attributes::deserialize_number_from_string"
    )]
    pub port: u16,
}

#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
}

impl Settings {
    #[inline]
    pub fn parse() -> Result<Self, Error> {
        let config_dir = env::current_dir()?.join("configuration");
        let settings = Config::builder()
            .add_source(config::File::from(config_dir.join("base.toml")))
            .add_source(
                config::Environment::with_prefix("APP")
                    .prefix_separator("_")
                    .separator("__"),
            )
            .build()?;

        let res = settings.try_deserialize()?;

        Ok(res)
    }
}
