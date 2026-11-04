mod config;
mod common;
mod security;
mod health;
mod user;

use axum::Router;
use config::{from_env, create_pool};
use tokio::runtime;
use tokio::net::TcpListener;
use std::{sync::Arc};
use crate::common::{repo::Repo, service::Service, routes};
use user::{user_repo::UserRepo, user_service::UserService};

use crate::{health::health_route, security::{aes::AES_KEY, hmac::HMAC_KEY}};

#[derive(Debug, thiserror::Error)]
pub enum AppErr {

    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ConfigErr(#[from] config::config::ConfigError),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),

    
}

fn main() -> Result<(),AppErr>{
    tracing_subscriber::fmt::init();

    let _ = &*HMAC_KEY;
    let _ = &*AES_KEY;

    let conf = from_env()?;

    let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;
    
    let repo = Arc::new(
        Repo::new( 
            runtime.block_on(
                create_pool(&conf))?));

    let repo_user = UserRepo::new(repo);
    let service_user = UserService::new(Service::new(repo_user));
    
    runtime.block_on(async {
        let listen = TcpListener::bind("0.0.0.0:8080").await?;
        let route = Router::new().merge(health_route());

        axum::serve(listen,route).await?;
        Ok::<(), std::io::Error>(())
    })?;

    Ok(())
}
