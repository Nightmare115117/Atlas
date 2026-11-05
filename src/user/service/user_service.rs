use crate::security::{aes, hmac, argon2id};
use crate::user::
{user_repo::{UserRepo},
user_model::{UserModel}};
use crate::common::service::{Service, ServiceG};

pub struct UserService {
    service: Service<UserRepo>
}

impl UserService {
    pub fn new(service: Service<UserRepo>) -> UserService {
        UserService { service: service}
    } 
}

impl ServiceG for UserService {
    
    async fn get_all(&self) -> Result<Vec<Self::Entity>, Self::Error> {

    }

    async fn get_by_id(&self, id: Uuid) -> Result<Self::Entity, Self::Error> {

    }

    async fn create(&self, data: Self::CreateData) -> Result<(), Self::Error> {

    }

    async fn update(&self, id: Uuid, data: Self::UpdateData) -> Result<(), Self::Error> {

    }

    async fn delete(&self, id: Uuid) -> Result<(), Self::Error> {

    }
}