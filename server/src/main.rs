use std::sync::Arc;
use aiba_core::event::{Event, EventType, Message};
use aiba_core::event::handlers::BroadcasterLive;
use aiba_core::model::twitch::TwitchBroadcasterStatus;

fn main() {
    let mut event_organiser = aiba_core::event::Organiser::new();
    let publisher = event_organiser.get_publisher();
    {
        let publisher_thread = publisher.clone();
        std::thread::spawn(move || {
            loop {
                publisher_thread.send(Event::BroadcasterLiveEvent(Message::new(TwitchBroadcasterStatus {
                    name: "broadcaster-name".to_owned(),
                    live: true,
                }))).expect("Uh oh");
                std::thread::sleep(std::time::Duration::from_millis(1000));
            }
        });
    }

    {
        let publisher_thread = publisher.clone();
        std::thread::spawn(move || {
            loop {
                publisher_thread.send(Event::BroadcasterLiveEvent(Message::new(TwitchBroadcasterStatus {
                    name: "broadcaster-name".to_owned(),
                    live: false,
                }))).expect("Uh oh");
                std::thread::sleep(std::time::Duration::from_millis(1500));
            }
        });
    }

    event_organiser.register_event_handler(EventType::BroadcasterLive, Arc::new(BroadcasterLive::new()));
    event_organiser.run();
}
