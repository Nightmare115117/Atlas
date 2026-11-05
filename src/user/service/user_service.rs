use crate::security::{Argon2IdError, EncryptionError, HMACError};
use crate::security::{decrypt_email};
use crate::user::{UserRepo, UserRepoError, UserDTO, ErrorConvertir, UserDTOCreate};
use crate::common::{Repository, Service, ServiceG};
use uuid::Uuid;

pub struct UserService {
    service: Service<UserRepo>
}

impl UserService {
    pub fn new(service: Service<UserRepo>) -> UserService {
        UserService { service: service}
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
            let dto = UserDTO::from(UserDTO { email: decrypt_email(user.get_email_ciphertext())?, first_name: user.get_first_name().to_string(), last_name: user.get_last_name().to_string() });
            vector.push(dto);
        }
        Ok(vector)
    }

    async fn get_by_id(&self, id: Uuid) -> Result<Self::Entity, Self::Error> {
        let result = self.service.repo().get_by_id(id).await?;
        Ok(UserDTO::from(UserDTO { email: decrypt_email(result.get_email_ciphertext())?, first_name: result.get_first_name().to_string(), last_name: result.get_last_name().to_string()}))
    }

    async fn create(&self, data: Self::CreateData) -> Result<(), Self::Error> {

    }

    async fn update(&self, id: Uuid, data: Self::UpdateData) -> Result<(), Self::Error> {

    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {

    }
}