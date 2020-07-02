mod api;
mod api_trait;
mod model;
mod query;

use api_trait::*;
use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Method, Request, Response, Server, StatusCode};
use lazy_static::lazy_static;
use query::*;
use std::collections::HashMap;
use std::sync::Mutex;

/// This is our service handler. It receives a Request, routes on its
/// path, and returns a Future of a Response.
async fn echo(req: Request<Body>) -> Result<Response<Body>, hyper::Error> {
    match (req.method(), req.uri().path()) {
        (&Method::POST, "/api") => {
            let body = hyper::body::to_bytes(req.into_body()).await?;
            println!("raw query: {:#?}", body);
            let query: RootQuery = serde_json::from_slice(&body.to_vec()).unwrap();
            println!("query: {:#?}", query);
            let root: api::RootQuery = Default::default();
            let ret = root.handle(query).await.unwrap();
            Ok(Response::new(Body::from(ret)))
        }

        // Return the 404 Not Found for other routes.
        _ => {
            let mut not_found = Response::default();
            *not_found.status_mut() = StatusCode::NOT_FOUND;
            Ok(not_found)
        }
    }
}

lazy_static! {
    pub static ref USERS: Mutex<HashMap<i32, model::User>> = Mutex::new(HashMap::new());
    pub static ref ARTICLES: Mutex<Vec<model::Article>> = Mutex::new(Vec::new());
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = ([127, 0, 0, 1], 9090).into();
    let service = make_service_fn(|_| async { Ok::<_, hyper::Error>(service_fn(echo)) });
    let server = Server::bind(&addr).serve(service);
    println!("Listening on http://{}", addr);
    server.await?;

    Ok(())
}
