use actix_web::{Error, HttpRequest, HttpResponse, web};
use log::{info};
use aiba_core::event::{Event, Message};
use aiba_core::model::twitch::TwitchBroadcasterStatus;
use crate::api::Context;

pub async fn new_event(req: HttpRequest, context : Context) -> Result<HttpResponse, Error> {
    if req.method().as_str() != "GET" {
        return Ok(HttpResponse::MethodNotAllowed().finish())
    }

    context.publisher.send(Event::BroadcasterLiveEvent(Message::new(TwitchBroadcasterStatus {
        name: "broadcaster-name".to_owned(),
        live: true,
    }))).expect("Uh oh");

    Ok(HttpResponse::Ok().finish())
}