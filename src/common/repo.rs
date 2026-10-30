use sqlx::{Pool, Postgres};
use uuid::Uuid;

pub struct Repo {
    pool: Pool<Postgres>,
}

impl Repo {
    pub fn new(pool: Pool<Postgres>) -> Repo {
        Repo {pool: pool}
    }

    pub fn get_pool(&self) -> &Pool<Postgres> {
        &self.pool
    }
}

pub trait Repository<T> {
    type Error;

    async fn get_all(&self) -> Result<Vec<T>, Self::Error>;
    async fn get_by_id(&self, id: Uuid) -> Result<T, Self::Error>;
    async fn create(&self, data: T) -> Result<(), Self::Error>;
    async fn update(&self, id: Uuid, data: T) -> Result<(), Self::Error>;
    async fn delete(&self, id: Uuid) -> Result<(),Self::Error>;
}