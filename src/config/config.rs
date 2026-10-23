use std::env::{
    VarError,
    var
};
use dotenvy::{dotenv, Error};

#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("Error al leer la variable de entorno: {0}")]
    Environment(#[from] VarError),

    #[error("Error al cargar el archivo .env: {0}")]
    Dotenv(#[from] Error),
}

pub struct Config {
    database_url: String
}

impl Config {
    pub fn get_db_url(&self) -> &str {
        &self.database_url
    }
}

pub fn from_env() -> Result<Config, ConfigError> {
    match var("DATABASE_URL") {
        Ok(res) => {Ok(Config { database_url: res })}
        Err(_) => {
            match dotenv() {
                Ok(path) => {
                    println!("La variable de entorno en la ruta: {path:?} a sido cargada correctamente");
                    match var("DATABASE_URL") {
                        Ok(res) => {Ok(Config { database_url: res })}
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