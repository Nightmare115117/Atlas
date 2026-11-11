use crate::common::{Repo, Repository};
use crate::model::OrgModel;

use sqlx::{Postgres, QueryBuilder, Row, query, query_as};
use std::sync::Arc;
use tracing::instrument;

pub struct OrgRepo {
    pool: Arc<Repo>,
}

impl OrgRepo {
    pub fn new(repo: Arc<Repo>) -> Self {
        OrgRepo { pool: repo }
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OrgRepoError {
    #[error("Error de base de datos: {0}")]
    Database(#[from] sqlx::Error),

    #[error("No hay campos para actualizar")]
    NoFieldsToUpdate,

    #[error("Ningún usuario fue actualizado")]
    NoRowAffected,
}

impl Repository<OrgRepo> for OrgRepo {
    type Error = OrgRepoError;

    async fn get_all(&self) -> impl Future<Output = Result<Vec<OrgRepo>, Self::Error>> {}
}
