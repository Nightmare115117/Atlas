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
    database_url: String,
    bcrypt_cost: u32,
}

impl Config {
    pub fn get_db_url(&self) -> &str {
        self.database_url.as_str()
    }

    pub fn get_bcrypt_cost(&self) -> u32 {
        self.bcrypt_cost
    }
}

pub fn from_env() -> Result<Config, ConfigError> {
    match var("DATABASE_URL") {
        Ok(res) => {Ok(Config { database_url: res, bcrypt_cost: 12 })}
        Err(_) => {
            match dotenv() {
                Ok(path) => {
                    println!("La variable de entorno en la ruta: {path:?} a sido cargada correctamente");
                    match var("DATABASE_URL") {
                        Ok(res) => {Ok(Config { database_url: res, bcrypt_cost: 12 })}
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