use crate::event::{Event, EventHandler};
use async_trait::async_trait;

pub struct BroadcasterLive;

impl BroadcasterLive {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl EventHandler for BroadcasterLive {
    async fn run(&self, event: Event) {
        if let Event::BroadcasterLiveEvent(data) = event {
            println!("{}, {} at {}", data.message.name, data.message.live, data.timestamp);
        }
    }
}