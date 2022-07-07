use std::sync::Arc;
use dashmap::DashMap;
use tokio::runtime::Runtime;
use tokio::sync::broadcast::{Sender, Receiver, channel};
use crate::model::twitch::TwitchBroadcasterStatus;
use async_trait::async_trait;

pub mod handlers;

#[derive(Clone, Debug)]
pub enum Event {
    BroadcasterLiveEvent(Message<TwitchBroadcasterStatus>),
    BroadcasterOfflineEvent(Message<TwitchBroadcasterStatus>),
}

#[derive(Clone, Debug)]
pub struct Message<T : Clone> {
    pub timestamp : u128,
    pub message : T
}

impl<T : Clone> Message<T> {
    pub fn get_message_ref(&self) -> &T {
        &self.message
    }

    pub fn get_message(&self) -> T {
        self.message.clone()
    }

    pub fn new(msg : T) -> Self {
        Self {
            message: msg,
            timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_millis()
        }
    }
}

impl Event {
    #[must_use]
    pub fn event_type(&self) -> EventType {
        match self {
            Event::BroadcasterLiveEvent(_) => EventType::BroadcasterLive,
            Event::BroadcasterOfflineEvent(_) => EventType::BroadcasterOffline
        }
    }
}

impl From<&Event> for EventType {
    fn from(event: &Event) -> EventType {
        event.event_type()
    }
}

#[derive(Clone)]
pub enum EventType {
    /// This maps to [`BroadcasterLiveEvent`].
    BroadcasterLive,
    /// This maps to [`BroadcasterOfflineEvent`].
    BroadcasterOffline
}

impl EventType {
    pub fn name(&self) -> String {
        match self {
            EventType::BroadcasterLive => "broadcaster_live".to_owned(),
            EventType::BroadcasterOffline => "broadcaster_offline".to_owned(),
        }
    }
}

#[async_trait]
pub trait EventHandler : Send + Sync {
    async fn run(&self, event : Event);
}

pub struct EventHandlerRegistry {
    handlers : DashMap<String, Vec<Arc<dyn EventHandler>>>
}

impl EventHandlerRegistry {
    pub fn add_event_handler(&self, e_type : EventType, handler : Arc<dyn EventHandler>) {
        if !self.handlers.contains_key(&e_type.name()) {
            self.handlers.insert(e_type.name(), Vec::new());
        }
        let mut handlers = self.handlers.get_mut(&e_type.name()).unwrap();
        handlers.push(handler);
    }

    pub fn new() -> Self {
        Self {
            handlers: DashMap::new()
        }
    }
}

pub struct Organiser {
    event_handler_registry : Arc<EventHandlerRegistry>,
    receiver: Receiver<Event>,
    sender : Sender<Event>
}

impl Organiser {
    pub fn register_event_handler(&self, e_type : EventType, handler : Arc<dyn EventHandler>) {
        self.event_handler_registry.add_event_handler(e_type, handler);
    }
    pub fn get_publisher(&self) -> Sender<Event> {
        self.sender.clone()
    }
    pub fn get_consumer(&self) -> Receiver<Event> {
        self.sender.subscribe()
    }

    pub fn run(&mut self) {
        let rt = new_runtime();
        rt.block_on( async move {
            loop {
                let resp = self.receiver.recv().await.unwrap();
                let handlers = self.event_handler_registry.handlers.get(&resp.event_type().name()).unwrap();
                println!("Event received: {}", resp.event_type().name());
                for handler in handlers.to_vec() {
                    handler.run(resp.clone()).await;
                }
            }
        })
    }

    pub fn new() -> Self {
        let (tx, rx) = channel(5000);
        Self {
            event_handler_registry: Arc::new(EventHandlerRegistry::new()),
            receiver: rx,
            sender: tx
        }
    }
}


fn new_runtime() -> Runtime {
    tokio::runtime::Builder::new_multi_thread()
        .thread_name("aiba_event_organiser")
        .enable_all()
        .build().unwrap()
}