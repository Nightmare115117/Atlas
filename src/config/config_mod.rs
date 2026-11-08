use std::env::{
    VarError,
    var
};
use dotenvy::{dotenv, Error};
use tracing::{info, instrument};

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Error al leer la variable de entorno: {0}")]
    Environment(#[from] VarError),

    #[error("Error al cargar el archivo .env: {0}")]
    Dotenv(#[from] Error),
}

#[derive(Debug)]
pub struct Config {
    database_url: String,
}

impl Config {
    pub fn get_db_url(&self) -> &str {
        self.database_url.as_str()
    }
}

#[instrument(err)]
pub fn from_env() -> Result<Config, ConfigError> {
    match var("DATABASE_URL") {
        Ok(res) => {
            info!("La variable de entorno a sido cargada correctamente");
            Ok(Config { database_url: res})
        }
        Err(_) => {
            match dotenv() {
                Ok(path) => {
                    info!("La variable de entorno en la ruta: {path:?} a sido cargada correctamente");
                    match var("DATABASE_URL") {
                        Ok(res) => {
                            info!("La variable de entorno en la ruta: {path:?} a sido cargada correctamente");
                            Ok(Config { database_url: res})}
                        Err(error ) => {
                            Err(ConfigError::Environment(error))
                        }
                    }
                }
                Err(err) => {
                    Err(ConfigError::Dotenv(err))
                }
            }
        }
    }
}