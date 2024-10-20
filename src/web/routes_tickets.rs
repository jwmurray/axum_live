use crate::model::{ModelController, Ticket, TicketForCreate};
use crate::Result;
use axum::extract::{FromRef, Path, State};
use axum::routing::{delete, post};
use axum::{Json, Router};
use serde_json::{json, Value};
use std::sync::Arc;

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket).get(list_tickets))
        .route("/tickets/:id", delete(delete_ticket))
        .with_state(mc)
}

// region:    --- Rest handlers
async fn create_ticket(
    mc: State<ModelController>,
    ticket_fc: Json<TicketForCreate>,
) -> Result<Json<Ticket>> {
    println!("->> {:<12} - create_ticket - {ticket_fc:?}", "HANDLER");

    let ticket = mc.create_ticket(ticket_fc.0).await?; // Call the create_ticket method

    Ok(Json(ticket))
}

async fn list_tickets(State(mc): State<ModelController>) -> Result<Json<Vec<Ticket>>> {
    println!("->> {:<12} - list_tickets", "HANDLER");

    let tickets = mc.list_tickets().await?; // Call the list_tickets method

    Ok(Json(tickets))
}

async fn delete_ticket(
    State(mc): State<ModelController>,
    Path(id): Path<u64>,
) -> Result<Json<Value>> {
    println!("->> {:<12} - delete_ticket - id: {id}", "HANDLER");

    mc.delete(id).await?; // Call the delete method

    Ok(Json(json!({ "result": { "success": true } })))
}
// endregion: --- Rest handlers
