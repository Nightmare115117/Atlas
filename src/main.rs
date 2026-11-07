mod common;
mod config;
mod health;
mod security;
mod user;

use axum::Router;
use config::{create_pool, from_env};
use std::sync::Arc;
use tokio::{net::TcpListener, runtime};

use crate::{
    common::{Repo, Route, Service, UserState},
    health::health_route,
    security::{aes::AES_KEY, hmac::HMAC_KEY},
    user::{UserRepo, UserRoutes, UserService},
};

#[derive(Debug, thiserror::Error)]
pub enum AppErr {
    #[error(transparent)]
    IoError(#[from] std::io::Error),

    #[error(transparent)]
    ConfigErr(#[from] config::config::ConfigError),

    #[error(transparent)]
    DatabaseError(#[from] sqlx::Error),
}

fn main() -> Result<(), AppErr> {
    tracing_subscriber::fmt::init();

    let _ = &*HMAC_KEY;
    let _ = &*AES_KEY;

    let conf = from_env()?;

    let runtime = runtime::Builder::new_multi_thread().enable_all().build()?;

    let repo = Arc::new(Repo::new(runtime.block_on(create_pool(&conf))?));

    let repo_user = UserRepo::new(repo);
    let service_user = UserService::new(Service::new(repo_user));
    let user_state = UserState {
        service: Arc::new(service_user),
    };
    let user_router = UserRoutes::routes().with_state(user_state);

    runtime.block_on(async move {
        let listen = TcpListener::bind("0.0.0.0:8080").await?;
        let route = Router::new()
            .merge(health_route())
            .nest("/api/user", user_router);

        axum::serve(listen, route).await?;
        Ok::<(), std::io::Error>(())
    })?;

    Ok(())
}
