use std::sync::Arc;
use aiba_core::event::{Event, EventType};
use aiba_core::event::handlers::BroadcasterLive;
use aiba_core::model::twitch::TwitchBroadcasterStatus;

fn main() {
    let mut event_organiser = aiba_core::event::Organiser::new();
    let publisher = event_organiser.get_publisher();
    std::thread::spawn(move || {
        loop {
            publisher.send(Event::BroadcasterLiveEvent(TwitchBroadcasterStatus {
                name: "broadcaster-name".to_string(),
                live: true
            })).expect("Uh oh");
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
    });
    event_organiser.register_event_handler(EventType::BroadcasterLive, Arc::new(BroadcasterLive::new()));
    event_organiser.run();
}
