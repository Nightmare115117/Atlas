use sqlx::{
    Error, 
    PgPool, 
    postgres::PgPoolOptions};
use tracing::instrument;
use crate::config::Config;


#[instrument(err)]
pub async fn create_pool(config: &Config) -> Result<PgPool, Error> {
    PgPoolOptions::new()
    .max_connections(5)
    .connect(config.get_db_url())
    .await
}