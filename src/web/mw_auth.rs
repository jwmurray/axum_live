use crate::{
    Error,
    Result,
    web::AUTH_TOKEN,
};

use axum::{
    body::Body,
    http::Request, 
    response::Response,
    middleware::{self, Next},
};


pub async fn mw_require_auth(
    // ctx: Result<Ctx>,
    req: Request<Body>, 
    next: Next,
) -> Result<Response> {
    println!("->> {:<12} - mw_require_auth", "MIDDLEWARE");

    Ok(next.run(req).await)
}