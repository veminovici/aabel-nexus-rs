use crate::{Aid, IntoAid, IntoSid, Sid};
use actix::{dev::MessageResponse, Actor, Message};
use std::fmt::{Display, LowerHex, UpperHex};

#[derive(Debug)]
pub struct StatsResponse {
    pub ttl_msgs: usize,
    pub ttl_nodes: usize,
}

impl StatsResponse {
    pub fn new(ttl_msgs: usize, ttl_nodes: usize) -> Self {
        Self {
            ttl_msgs,
            ttl_nodes,
        }
    }
}

impl<A> MessageResponse<A, StatsMessage> for StatsResponse
where
    A: Actor,
{
    fn handle(
        self,
        _ctx: &mut <A as actix::prelude::Actor>::Context,
        tx: Option<actix::prelude::dev::OneshotSender<<StatsMessage as Message>::Result>>,
    ) {
        if let Some(tx) = tx {
            tx.send(self).unwrap()
        }
    }
}

impl Display for StatsResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "msgs: {}, nodes={}", self.ttl_msgs, self.ttl_msgs)
    }
}

pub struct StatsMessage {
    sid: Sid,
    aid: Aid,
}

impl StatsMessage {
    const MSG_ID: &'static str = "STAT";
}

impl Message for StatsMessage {
    type Result = StatsResponse;
}

//
// Display, UpperHex, LowerHex traits
//

impl Display for StatsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ▫︎ {} ▫︎ {}", Self::MSG_ID, self.sid, self.aid)
    }
}

impl UpperHex for StatsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ▫︎ {} ▫︎ {}", Self::MSG_ID, self.sid, self.aid)
    }
}

impl LowerHex for StatsMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ▫︎ {} ▫︎ {}", Self::MSG_ID, self.sid, self.aid)
    }
}

//
// Message builders
//

pub struct StatsBuilder;

impl StatsBuilder {
    pub fn with_sid<S>(identifier: S) -> StatsBuilderWithSid
    where
        S: IntoSid,
    {
        StatsBuilderWithSid {
            sid: identifier.into_sid(),
        }
    }
}

pub struct StatsBuilderWithSid {
    sid: Sid,
}

impl StatsBuilderWithSid {
    pub fn with_aid<A>(self, identifier: A) -> StatsMessage
    where
        A: IntoAid,
    {
        StatsMessage {
            sid: self.sid,
            aid: identifier.into_aid(),
        }
    }
}
