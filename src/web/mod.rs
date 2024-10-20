pub mod mw_auth;
pub mod routes_login;
pub mod routes_tickets; // This line is incorrect, it should be `pub mod routes_tickets;`

pub const AUTH_TOKEN: &str = "auth-token";
