use actix_web::{Error, HttpRequest, HttpResponse, web};
use log::{info};
use aiba_core::event::{Event, EventEnvelope, Message};
use aiba_core::model::twitch::TwitchBroadcasterStatus;
use crate::api::Context;

pub async fn new_event(req: HttpRequest, context : Context, body : web::Bytes) -> Result<HttpResponse, Error> {
    if req.method().as_str() != "GET" {
        return Ok(HttpResponse::MethodNotAllowed().finish())
    }

    let payload : EventEnvelope = serde_json::from_slice(&body).map_err(actix_web::error::ErrorBadRequest)?;
    context.publisher.send(payload.payload).expect("Uh oh");

    Ok(HttpResponse::Ok().finish())
}