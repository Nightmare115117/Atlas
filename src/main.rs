mod config;
mod health;

use config::{from_env, create_pool};
use tokio::runtime;
use tokio::net::TcpListener;
use std::process::exit;
use hyper::service::service_fn;
use hyper::body::Incoming;
use hyper_util::{rt::{TokioExecutor, TokioIo}, server::conn::auto::Builder};
use hyper::Request;
use tower::ServiceExt;

use crate::health::health_route;

fn main() {

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
    
    let pool = match runtime.block_on(create_pool(&conf)) {
        Ok(res) => {
            println!("El pool se a creado correctamente");
            res
        }
        Err(error) => {
            eprintln!("Error al crear la pool de la base de datos, error: {error}");
            exit(1);
        }
    };

    runtime.block_on(async {
        let listen = match TcpListener::bind("127.0.0.1:8080").await {
            Ok(res) => {res}
            Err(error) => {
                eprintln!("Error al crear el Listener, error: {error}");
                exit(1)
            }
        };
        
        let route = health_route();

        loop {
            let (stream, addr) = match listen.accept().await {
                Ok(res) => {res}
                Err(error) => {
                    eprintln!("Error al obtener los valores, error: {error}");
                    exit(1);
                }
            };

            let io = TokioIo::new(stream);
            
            let clone = route.clone();
            let service = service_fn(move |request: Request<Incoming>| {
                let route_clone = clone.clone();

                async move {
                    let request = request.map(axum::body::Body::new);
                    route_clone.oneshot(request).await
                }
            });

            
            
            tokio::spawn(async move {
                let builder = Builder::new(TokioExecutor::new())
                .http2_only();
                if let Err(error) = builder.serve_connection(io, service).await {
                    eprintln!("Error en la conexión con {addr}: {error}");
                }
            });
        }
    })
}
