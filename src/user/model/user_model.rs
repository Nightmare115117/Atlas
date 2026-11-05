use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(FromRow, Debug)]
pub struct UserModel {
    id: Uuid,
    pub email: String,
    first_name: String,
    last_name: String,
    email_ciphertext: String,
    email_hmac: String,
    password_hash: String,
    status: String,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
}

impl UserModel {

    pub fn new() -> UserModel {
        UserModel { id: Uuid::new_v4(), email: String::new(), first_name: String::new(), last_name: String::new(), email_ciphertext: String::new(), email_hmac: String::new(), password_hash: String::new(), status: String::new(), created_at: OffsetDateTime::now_utc(), updated_at: OffsetDateTime::now_utc() }
    }

    pub fn get_first_name(&self) -> &str {
        &self.first_name
    }

    pub fn set_first_name(&mut self, value: &str) {
        self.first_name = value.to_owned();
    }

    pub fn get_last_name(&self) -> &str {
        &self.last_name
    }

    pub fn set_last_name(&mut self, value: &str) {
        self.last_name = value.to_owned();
    }

    pub fn get_email_ciphertext(&self) -> &str {
        &self.email_ciphertext
    }

    pub fn set_email_ciphertext(&mut self, value: &str) {
        self.email_ciphertext = value.to_owned();
    }

    pub fn get_email_hmac(&self) -> &str {
        &self.email_hmac
    }

    pub fn set_email_hmac(&mut self, value: &str) {
        self.email_hmac = value.to_owned();
    }

    pub fn get_password_hash(&self) -> &str {
        &self.password_hash
    }

    pub fn set_password_hash(&mut self, value: &str) {
        self.password_hash = value.to_owned();
    }

    pub fn get_status(&self) -> &str {
        &self.status
    }

    pub fn set_status(&mut self, value: &str) {
        self.status = value.to_owned();
    }

    pub fn get_created_at(&self) -> &OffsetDateTime {
        &self.created_at
    }

    pub fn set_created_at(&mut self, value: &OffsetDateTime) {
        self.created_at = *value;
    }

    pub fn get_updated_at(&self) -> &OffsetDateTime {
        &self.updated_at
    }

    pub fn set_updated_at(&mut self, value: &OffsetDateTime) {
        self.updated_at = *value;
    }
}

#[derive(Serialize, Deserialize)]
pub struct UserDTO {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorConvertir {
    #[error("Correo electrónico inválido")]
    EmailInvalido,

    #[error("Datos incompletos")]
    DatosIncompletos,

    #[error("El usuario está vacío")]
    UsuarioVacio,

    #[error("Error interno: {0}")]
    ErrorInterno(String),
}

impl TryFrom<Option<UserDTO>> for UserModel {
    type Error  = ErrorConvertir;

    fn try_from(user: Option<UserDTO>) -> Result<Self, Self::Error> {
        let usuario = match user {
            Some(usuario) => usuario,
            None => return Err(Self::Error::UsuarioVacio),
        };

        if usuario.email.trim().is_empty() {
            Err(Self::Error::EmailInvalido)
        } else {
            Ok(UserModel { id: Uuid::new_v4(), email: usuario.email, first_name: usuario.first_name, last_name: usuario.last_name, email_ciphertext: String::new(), email_hmac: String::new(), password_hash: String::new(), status: String::new(), created_at: OffsetDateTime::now_utc(), updated_at: OffsetDateTime::now_utc() })
        }
    }
} 

impl From<UserModel> for UserDTO {
    fn from(usuario: UserModel) -> Self {
        UserDTO {
            email: usuario.email,
            first_name: usuario.first_name,
            last_name: usuario.last_name,
        }
    }
}

pub struct UserDTOCreate {
    pub email: String, 
    pub password: String,
    pub fist_name: String,
    pub last_name: String,
}