use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TwitchBroadcasterStatus {
    pub name : String,
    pub live : bool
}