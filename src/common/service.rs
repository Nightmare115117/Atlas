use uuid::Uuid;

pub struct Service <R> {
    repo: R,
}

impl<R> Service<R> {
    pub fn new(repo: R) -> Service<R> {
        Service{repo: repo}
    }

    pub fn repo(&self) -> &R {
        &self.repo
    }
}

pub trait ServiceG {
    type Entity;
    type CreateData;
    type UpdateData;
    type Error;

    async fn get_all(&self) -> Result<Vec<Self::Entity>, Self::Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<Self::Entity, Self::Error>;
    async fn create(&self, data: Self::CreateData) -> Result<(), Self::Error>;
    async fn update(&self, id: Uuid, data: Self::UpdateData) -> Result<(), Self::Error>;
    async fn delete(&self, id: Uuid) -> Result<(), Self::Error>;
}