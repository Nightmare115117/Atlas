mod config;
mod common;
mod security;
mod health;
mod user;

use axum::Router;
use tracing::instrument;
use config::{from_env, create_pool};
use tokio::runtime;
use tokio::net::TcpListener;
use std::{sync::Arc};
use common::{repo, service, routes};
use user::user_model;

use crate::{common::repo::Repo, health::health_route, security::{aes::AES_KEY, hmac::HMAC_KEY}, user::repo::user_repo::UserRepo};

#[derive(Debug,thiserror::Error)]
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

    runtime.block_on(async {
        let listen = TcpListener::bind("0.0.0.0:8080").await?;
        
        let route = Router::new().merge(health_route());

        //loop {
        //    let (stream, addr) = match listen.accept().await {
        //        Ok(res) => {res}
        //        Err(error) => {
        //            eprintln!("Error al obtener los valores, error: {error}");
        //            exit(1);
        //        }
        //    };

        //    let io = TokioIo::new(stream);
            
        //    let clone = route.clone();
        //    let service = service_fn(move |request: Request<Incoming>| {
        //        let route_clone = clone.clone();

        //        async move {
        //            let request = request.map(axum::body::Body::new);
        //            route_clone.oneshot(request).await
        //        }
        //    });
    
        //    tokio::spawn(async move {
        //        let builder = Builder::new(TokioExecutor::new())
        //        .http2_only();
        //        if let Err(error) = builder.serve_connection(io, service).await {
        //            eprintln!("Error en la conexión con {addr}: {error}");
        //        }
        //    });
        //}

        axum::serve(listen,route).await?;
        Ok::<(), std::io::Error>(())
    })?;

    Ok(())
}
