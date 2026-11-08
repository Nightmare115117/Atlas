use crate::common::{Repository, Service, ServiceG};
use crate::security::{
    Argon2IdError, EncryptionError, HMACError, decrypt_email, encrypt_email, hash_password,
    sign_email, verify_password,
};
use crate::user::{ErrorConvertir, UserDTO, UserDTOCreate, UserModel, UserRepo, UserRepoError};
use uuid::Uuid;

#[derive(Debug)]
pub struct UserService {
    pub service: Service<UserRepo>,
}

impl UserService {
    pub fn new(servicio: Service<UserRepo>) -> UserService {
        UserService { service: servicio }
    }
}

impl UserService {
    pub async fn login(&self, email: &str, password: &str) -> Result<bool, UserServiceError> {
        let (_, password_hash) = self
            .service
            .repo()
            .get_by_email(&sign_email(email)?)
            .await?;

        let result = verify_password(password, &password_hash)?;
        Ok(result)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error(transparent)]
    Repository(#[from] UserRepoError),

    #[error(transparent)]
    PasswordHash(#[from] Argon2IdError),

    #[error(transparent)]
    EmailEncryption(#[from] EncryptionError),

    #[error(transparent)]
    EmailHMAC(#[from] HMACError),

    #[error(transparent)]
    UserParse(#[from] ErrorConvertir),

    #[error("Usuario no encontrado")]
    UserNotFound,

    #[error("Datos invalidos")]
    InvalidData,
}

impl ServiceG for UserService {
    type Entity = UserDTO;
    type CreateData = UserDTOCreate;
    type Error = UserServiceError;
    type UpdateData = UserDTOCreate;

    async fn get_all(&self) -> Result<Vec<Self::Entity>, Self::Error> {
        let result = self.service.repo().get_all().await?;
        let mut vector: Vec<UserDTO> = Vec::new();
        for user in result.iter() {
            let dto = UserDTO {
                email: decrypt_email(user.get_email_ciphertext())?,
                first_name: user.get_first_name().to_string(),
                last_name: user.get_last_name().to_string(),
            };
            vector.push(dto);
        }
        Ok(vector)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Self::Entity, Self::Error> {
        let result = self.service.repo().get_by_id(id).await?;
        Ok(UserDTO {
            email: decrypt_email(result.get_email_ciphertext())?,
            first_name: result.get_first_name().to_string(),
            last_name: result.get_last_name().to_string(),
        })
    }

    async fn create(&self, data: Self::CreateData) -> Result<(), Self::Error> {
        let mut user = UserModel::new();
        user.set_email_ciphertext(&encrypt_email(&data.email)?);
        user.set_email_hmac(&sign_email(&data.email)?);
        user.set_password_hash(&hash_password(&data.password)?);
        user.set_first_name(&data.first_name);
        user.set_last_name(&data.last_name);

        self.service.repo().create(user).await?;
        Ok(())
    }

    async fn update(&self, id: Uuid, data: Self::UpdateData) -> Result<(), Self::Error> {
        let mut user = UserModel::new();

        if !data.email.trim().is_empty() {
            user.set_email_ciphertext(&encrypt_email(&data.email)?);
            user.set_email_hmac(&sign_email(&data.email)?);
        }

        if !data.password.trim().is_empty() {
            user.set_password_hash(&hash_password(&data.password)?);
        }

        if !data.first_name.trim().is_empty() {
            user.set_first_name(&data.first_name);
        }

        if !data.last_name.trim().is_empty() {
            user.set_last_name(&data.last_name);
        }

        if !data.status.trim().is_empty() {
            user.set_status(&data.status);
        }

        self.service.repo().update(id, user).await?;
        Ok(())
    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {
        self.service.repo().delete(id).await?;
        Ok(())
    }
}
