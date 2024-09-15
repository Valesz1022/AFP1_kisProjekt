//! Konfigurációs lehetőségek beolvasására és tárolására szolgáló modul.

use std::{env, io, path::Path};

use config::{Config, ConfigError};
use serde::Deserialize;
use serde_aux::field_attributes;
use sqlx::mysql::MySqlConnectOptions;
use thiserror::Error;

/// Általános konfigurációs lehetőségek az applikációhoz.
#[non_exhaustive]
#[derive(Deserialize, Debug)]
pub struct ApplicationSettings {
    /// A host, amelyen futni fog a szerver.
    pub host: String,
    #[serde(
        deserialize_with = "field_attributes::deserialize_number_from_string"
    )]
    /// A port, amelyen futni fog a szerver a megadott hoston.
    pub port: u16,
}

/// Konfigurációs lehetőségek az adatbázishoz való kapcsolódáshoz.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct DatabaseSettings {
    /// Felhasználónév, amellyel a szerver csatlakozni fog az adatbázishoz.
    pub username: String,
    /// Jelszó, amely a megadott felhasználónévhez tartozik.
    pub password: String,
    /// A futó adatbáziskezelőrendszer megnyitott portja, ahol lehet csatlakozni.
    #[serde(
        deserialize_with = "field_attributes::deserialize_number_from_string"
    )]
    pub port: u16,
    /// A futó adatbázisrendszer megnyitott hostneve, ahonnan elérhető.
    pub host: String,
    /// Az adatbáziskezelőrendszeren belül található adatbázis neve, amihez
    /// a szerver csatlakozni fog.
    pub database_name: String,
}

impl DatabaseSettings {
    /// Sqlx-kompatibilis beállításokra konvertálva az elmentett konfiguráció.
    #[must_use]
    #[inline]
    pub fn connect_options(&self) -> MySqlConnectOptions {
        MySqlConnectOptions::new()
            .host(&self.host)
            .username(&self.username)
            .password(&self.password)
            .port(self.port)
            .database(&self.database_name)
    }
}

/// A konfigurációs beállítások beolvasása közben felmerülhető problémák.
#[non_exhaustive]
#[derive(Debug, Error)]
pub enum ReadError {
    /// Nem sikerült megtalálni a jelenlegi mappát.
    #[error("Failed to determine the current working directory.")]
    Cwd(#[from] io::Error),
    /// Nem sikerült beolvasni a konfigurációs fájlban található adatokat.
    #[error("Failed to read configuration.")]
    Config(#[from] ConfigError),
}

/// Egybefogó konfigurációs lehetőségek a szerverhez.
///
/// Külön struktúra, hiszen így kényelmesebb a konfigurációs fájl elkészítése.
#[non_exhaustive]
#[derive(Debug, Deserialize)]
pub struct Settings {
    pub application: ApplicationSettings,
    pub database: DatabaseSettings,
}

impl Settings {
    /// A beállított konfigurációs lehetőségek beolvasása.
    ///
    /// A jelenlegi könyvtárban keres egy "configuration" alkönyvtárat,
    /// és azon belül keresi meg a megadott fájlnevet.
    #[inline]
    pub fn parse<P>(filename: P) -> Result<Self, ReadError>
    where
        P: AsRef<Path>,
    {
        let config_dir = env::current_dir()?.join("configuration");
        let settings = Config::builder()
            .add_source(config::File::from(config_dir.join(filename)))
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
