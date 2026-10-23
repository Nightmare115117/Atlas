use sqlx::{
    Error, 
    PgPool, 
    postgres::PgPoolOptions};
use crate::config::Config;

pub async fn create_pool(config: &Config) -> Result<PgPool, Error> {
    PgPoolOptions::new()
    .max_connections(5)
    .connect(config.get_db_url())
    .await
}