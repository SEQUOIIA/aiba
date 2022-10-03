use std::sync::Arc;
use aiba_core::event::{Event, EventType, Message};
use aiba_core::event::handlers::BroadcasterLive;
use aiba_core::model::twitch::TwitchBroadcasterStatus;

mod api;

fn main() {
    setup_tracing(Some(tracing::Level::TRACE));

    // TODO: Instead of process exit, send event via event_organiser to allow graceful shutdown
    ctrlc::set_handler(|| {
       std::process::exit(1);
    }).unwrap()
    ;
    let mut event_organiser = aiba_core::event::Organiser::new();

    let api_publisher = event_organiser.get_publisher();
    std::thread::spawn(|| {
       api::start_server(api_publisher);
    });

    // let publisher = event_organiser.get_publisher();
    // {
    //     let publisher_thread = publisher.clone();
    //     std::thread::spawn(move || {
    //         loop {
    //             publisher_thread.send(Event::BroadcasterLiveEvent(Message::new(TwitchBroadcasterStatus {
    //                 name: "broadcaster-name".to_owned(),
    //                 live: true,
    //             }))).expect("Uh oh");
    //             std::thread::sleep(std::time::Duration::from_millis(1000));
    //         }
    //     });
    // }
    //
    // {
    //     let publisher_thread = publisher.clone();
    //     std::thread::spawn(move || {
    //         loop {
    //             publisher_thread.send(Event::BroadcasterLiveEvent(Message::new(TwitchBroadcasterStatus {
    //                 name: "broadcaster-name".to_owned(),
    //                 live: false,
    //             }))).expect("Uh oh");
    //             std::thread::sleep(std::time::Duration::from_millis(1500));
    //         }
    //     });
    // }

    event_organiser.register_event_handler(EventType::BroadcasterLive, Arc::new(BroadcasterLive::new()));
    event_organiser.run();
}

fn setup_tracing(log_level: Option<tracing::Level>) {
    let max_level = log_level.unwrap_or(tracing::Level::TRACE);
    let filter = tracing_subscriber::EnvFilter::from_default_env()
        .add_directive(tracing::Level::INFO.into())
        .add_directive(
            format!("aiba_server={}", max_level.as_str().to_lowercase())
                .parse()
                .unwrap(),
        )
        .add_directive("reqwest=info".parse().unwrap())
        .add_directive("mio=info".parse().unwrap())
        .add_directive("want=info".parse().unwrap())
        .add_directive("actix_web=info".parse().unwrap())
        .add_directive("hyper=info".parse().unwrap());

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(max_level)
        .with_env_filter(filter)
        .finish();
    tracing_log::LogTracer::init().unwrap();
    tracing::subscriber::set_global_default(subscriber).unwrap();
}
