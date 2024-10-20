use crate::{web, Error, Result};
use axum::{routing::post, Json, Router, debug_handler};
use serde::Deserialize;
use serde_json::{json, Value};
use tower_cookies::{Cookie, Cookies};

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

#[debug_handler]
async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) -> Result<Json<Value>> {
    println!("->> {:<12} - api_login - {payload:?}", "HANDLER");

    if payload.username != "demo1" || payload.pwd != "welcome" {
        return Err(Error::LoginFail);
    }

    // Implement real auth-token generation/signature.
    cookies.add(Cookie::new(web::AUTH_TOKEN, "user-1.exp.sign"));

    // Create the success body.
    let body = Json(json!({
        "result": {
            "success": true,
        }
    }));

    Ok(body)
}

#[derive(Debug, serde::Deserialize)]
struct LoginPayload {
    username: String,
    pwd: String,
}
