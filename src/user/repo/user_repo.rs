use crate::repo::{Repository, Repo};
use crate::user_model::{UserModel};

use sqlx::{Postgres, QueryBuilder, query, query_as, Row};
use std::sync::Arc;

pub struct UserRepo {
    repo: Arc<Repo>,
}

impl UserRepo {
    pub fn new (repo :Arc<Repo>) -> UserRepo {
        UserRepo { repo: repo }
    } 

    pub async fn get_by_email(&self, correo: &str) -> Result<(String, String), UserRepoError> {
        let consulta = "SELECT email_ciphertext, password_hash FROM users WHERE email_hmac = $1";

        let result = query(consulta)
        .bind(correo)
        .fetch_optional(self.repo.get_pool())
        .await?;

        match result {
            Some(row) => Ok((row.try_get("email_ciphertext")?, row.try_get("password_hash")?)),
            None => Err(UserRepoError::UserNotFound)
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserRepoError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("No hay campos para actualizar")]
    NoFieldsToUpdate,

    #[error("Usuario no encontrado")]
    UserNotFound,

    #[error("Ningún usuario fue actualizado")]
    NoRowAffected,
}

impl Repository<UserModel> for UserRepo {
    type Error = UserRepoError;

    async fn get_all(&self) -> Result<Vec<UserModel>, Self::Error> {
       let consulta = "SELECT id, email_ciphertext, status, created_at, updated_at FROM users";
       query_as::<_,UserModel>(consulta)
       .fetch_all(self.repo.get_pool())
       .await
       .map_err(UserRepoError::Database)
    }

    async fn get_by_id(&self, id: uuid::Uuid) -> Result<UserModel, Self::Error> {
        let consulta = "SELECT id, email_ciphertext, status, created_at, updated_at FROM users WHERE id = $1";
        let result = query_as::<_, UserModel>(consulta)
        .bind(id)
        .fetch_optional(self.repo.get_pool())
        .await?;

        match result {
            Some(data) => Ok(data),
            None => Err(UserRepoError::UserNotFound)
        }
    }

    async fn create(&self, data: UserModel) -> Result<(), Self::Error> {
        let command = "INSERT INTO users (email_ciphertext, email_hmac, password_hash, status) VALUES ($1, $2, $3, $4)";
        query(command)
        .bind(data.get_email_ciphertext())
        .bind(data.get_email_hmac())
        .bind(data.get_password_hash())
        .bind("Pending")
        .execute(self.repo.get_pool())
        .await
        .map(|_| ())
        .map_err(UserRepoError::Database)
    }

    async  fn update(&self, id: uuid::Uuid, data: UserModel) -> Result<(), Self::Error> {
        let mut it: u8 = 0;
        let mut query = QueryBuilder::<Postgres>::new("UPDATE users SET ");
        {
            let mut separated = query.separated(", ");

            if !data.email.trim().is_empty() {
                separated.push("email_ciphertext = ")
                    .push_bind_unseparated(data.get_email_ciphertext());

                separated.push("email_hmac = ")
                    .push_bind_unseparated(data.get_email_hmac());

                it += 2;
            }

            if !data.get_password_hash().trim().is_empty() {
                separated.push("password_hash = ")
                    .push_bind_unseparated(data.get_password_hash());

                it += 1;
            }

            if !data.get_status().trim().is_empty() {
                separated.push("status = ")
                    .push_bind_unseparated(data.get_status());

                it += 1;
            }
        }
        query.push(" WHERE id = ");
        query.push_bind(id);

        if it == 0 { 
            return Err(UserRepoError::NoFieldsToUpdate);
        }

        let result = query.build()
        .execute(self.repo.get_pool())
        .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            Err(UserRepoError::NoRowAffected)
        }
    }

    async  fn delete(&self, id: uuid::Uuid) -> Result<(),Self::Error> {
        let command = "UPDATE users SET status = 'Inactive' WHERE id = $1";

        let result = query(command)
        .bind(id)
        .execute(self.repo.get_pool())
        .await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            return Err(UserRepoError::NoRowAffected);
        }
    }
}