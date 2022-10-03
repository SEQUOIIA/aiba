use std::sync::Arc;
use actix_web::{App, HttpServer, web};
use aiba_core::event::{Event, Organiser};
use crate::api::debug::new_event;
use crate::api::middleware::DefaultHeaders;
use tokio::sync::broadcast::{Sender, Receiver, channel};

pub mod middleware;
pub mod debug;

pub type Context = web::Data<ContextInner>;

#[actix_web::main]
pub async fn start_server(publisher : Sender<Event>) {
    let context = ContextInner {
        publisher
    };
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(context.clone()))
            // enable logger
            .wrap(actix_web::middleware::Logger::default())
            .wrap(DefaultHeaders)
            .service(web::scope("/debug")
                .service(web::resource("/new_event").to(new_event)))
    })
        .bind(("0.0.0.0", 8080)).unwrap()
        .run()
        .await.unwrap();
}

#[derive(Clone)]
pub struct ContextInner {
    pub publisher : Sender<Event>
}