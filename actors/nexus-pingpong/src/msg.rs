use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub(crate) enum PingPongMsg {
    Ping(u8),
    Pong(u8),
}
