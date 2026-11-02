mod config;
mod common;
mod security;
mod health;
mod user;

use axum::Router;
use config::{from_env, create_pool};
use tokio::runtime;
use tokio::net::TcpListener;
use std::{process::exit, sync::Arc};
use common::{repo, service, routes};
use security::bcrypt;
use user::user_model;

use crate::{common::repo::Repo, health::health_route, security::{aes::AES_KEY, hmac::HMAC_KEY}, user::repo::user_repo::UserRepo};

fn main() {

    let _ = &*HMAC_KEY;
    let _ = &*AES_KEY;

    let conf = match from_env() {
        Ok(res) => {res}
        Err(error) => {
            eprintln!("Error al configurar, error: {error}");
            exit(1);
        }
    };

    let runtime = match runtime::Builder::new_multi_thread().enable_all().build() {
        Ok(res) => {res}
        Err(error) => {
            eprintln!("Error al instanciar el runtime, error {error}");
            exit(1);
        }
    };
    
    let repo = Arc::new(Repo::new(match runtime.block_on(create_pool(&conf)) {
        Ok(res) => {
            println!("El pool se a creado correctamente");
            res
        }
        Err(error) => {
            eprintln!("Error al crear la pool de la base de datos, error: {error}");
            exit(1);
        }
    }));

    let repo_user = UserRepo::new(repo);

    runtime.block_on(async {
        let listen = match TcpListener::bind("127.0.0.1:8080").await {
            Ok(res) => {res}
            Err(error) => {
                eprintln!("Error al crear el Listener, error: {error}");
                exit(1)
            }
        };
        
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

        match axum::serve(listen,route).await {
            Ok(_) => {}
            Err(error) => {
                eprintln!("Error durante el funcionamiento del servidor error: {error}");
            }
        };
    })
}
