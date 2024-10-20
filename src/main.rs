#![allow(unused)]

pub use self::error::{Error, Result};

use std::net::SocketAddr;

use axum::{
    extract::{Path, Query, Extension, FromRequestParts},
    http::{Method, Request, Uri},
    middleware::{self, Next},
    response::{Html, IntoResponse, Response},
    routing::{get, get_service, post},
    Json, Router,
    body::Body,
    debug_handler,
};

use tokio::net::TcpListener;
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;

mod error;
mod model;
mod web;


#[tokio::main]
async fn main() -> Result<()> {
    // Initialize ModelController
    let mc = model::ModelController::new().await?;

    let routes_all = Router::new()
        .merge(routes_hello())
        .merge(web::routes_login::routes())
        .nest("/api", web::routes_tickets::routes(mc.clone()))
        .layer(middleware::map_response(main_response_mapper))
        .layer(CookieManagerLayer::new())
        .fallback_service(routes_static());

    let app = Router::new().merge(routes_all);

    // region: -- Start Server
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    println!("->> Listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
    Ok(())

    // endregion: -- Start Server
}

async fn main_response_mapper(res: Response) -> Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

    println!();
    res
}

fn routes_static() -> Router {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

#[derive(Debug, serde::Deserialize)]
struct HelloParams {
    name: Option<String>,
}

// region: --- Routes Hello
fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello/:name", get(handler_hello2))
}

// e.g., `/hello?name=Jennifer`
#[debug_handler]
async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World");
    Html(format!("Hello, <strong>{name}</strong>"))
}

// e.g., `/hello2/Mike`
#[debug_handler]
async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    Html(format!("Hello, <strong>{name}</strong>"))
}
// region:    --- Section
// region:    --- Section


// #![allow(unused)]

// pub use self::error::{Error, Result};

// use std::net::SocketAddr;

// use axum::{
//     extract::{Path, Query, Extension, FromRequestParts},
//     http::{Method, Request, Uri},
//     middleware::{self, Next},
//     response::{Html, IntoResponse, Response},
//     routing::{get, get_service, post},
//     Json, Router,
//     body::Body,
// };

// use tokio::net::TcpListener;
// use tower_cookies::CookieManagerLayer;
// use tower_http::services::ServeDir;

// mod error;
// mod model;
// mod web;

// async fn inject_path(req: Request<Body>, next: Next<Body>) -> Result<Response, axum::Error> {
//     let path = req.uri().path().to_string();
//     let (mut parts, body) = req.into_parts();
//     parts.extensions.insert(path);
//     let req = Request::from_parts(parts, body);
//     Ok(next.run(req).await)
// }

// // async fn inject_path<B>(req: Request<B>, next: Next) -> Result<Response> {
// //     let path = req.uri().path().to_string();
// //     let mut req_parts = <dyn axum::extract::FromRequestParts<B>>::new(req);
// //     req_parts.extensions_mut().insert(path);
// //     Ok(next.run(req_parts).await)
// // }

// #[tokio::main]
// async fn main() -> Result<()> {
//     // Initialize ModelController
//     let mc = model::ModelController::new().await?;

//     let routes_all = Router::new()
//         .merge(routes_hello())
//         .merge(web::routes_login::routes())
//         .nest("/api", web::routes_tickets::routes(mc.clone()))
//         .layer(middleware::map_response(main_response_mapper))
//         .layer(CookieManagerLayer::new())
//         .fallback_service(routes_static());

//     let app = Router::new().merge(routes_all);

//     // region: -- Start Server
//     let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
//     println!("->> Listening on http://{}", addr);
//     let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
//     axum::serve(listener, app).await.unwrap();
//     Ok(())

//     // endregion: -- Start Server
// }

// async fn main_response_mapper(res: Response) -> Response {
//     println!("->> {:<12} - main_response_mapper", "RES_MAPPER");

//     println!();
//     res
// }

// fn routes_static() -> Router {
//     Router::new().nest_service("/", get_service(ServeDir::new("./")))
// }

// #[derive(Debug, serde::Deserialize)]
// struct HelloParams {
//     name: Option<String>,
// }

// // region: --- Routes Hello
// fn routes_hello() -> Router {
//     Router::new()
//         .route("/hello", get(handler_hello))
//         .route("/hello/:name", get(handler_hello2))
// }

// // e.g., `/hello?name=Jennifer`
// async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
//     println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

//     let name = params.name.as_deref().unwrap_or("World");
//     Html(format!("Hello, <strong>{name}</strong>"))
// }

// // e.g., `/hello2/Mike`
// async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
//     println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

//     Html(format!("Hello, <strong>{name}</strong>"))
// }
// // region:    --- Section
// // region:    --- Section
