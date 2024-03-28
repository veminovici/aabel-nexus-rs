use crate::{Aid, DeliverMessage, IntoAid, IntoSid, Sid};
use actix::{Message, WeakRecipient};
use std::fmt::{Display, LowerHex, UpperHex};

pub struct RegNodeMessage {
    sid: Sid,
    aid: Aid,
    recepient: WeakRecipient<DeliverMessage>,
}

impl RegNodeMessage {
    const MSG_ID: &'static str = "RGND";

    pub fn aid(&self) -> &Aid {
        &self.aid
    }

    pub fn recepient(&self) -> &WeakRecipient<DeliverMessage> {
        &self.recepient
    }
}

impl Message for RegNodeMessage {
    type Result = ();
}

//
// Display, UpperHex, LowerHex traits
//

impl Display for RegNodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ▫︎ {} ▫︎ {}", Self::MSG_ID, self.sid, self.aid)
    }
}

impl UpperHex for RegNodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ▫︎ {} ▫︎ {}", Self::MSG_ID, self.sid, self.aid)
    }
}

impl LowerHex for RegNodeMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ▫︎ {} ▫︎ {}", Self::MSG_ID, self.sid, self.aid)
    }
}

//
// Message builders
//

pub struct RegBuilder;

impl RegBuilder {
    pub fn with_sid<S>(sid: S) -> RegBuilderWithSid
    where
        S: IntoSid,
    {
        RegBuilderWithSid {
            sid: sid.into_sid(),
        }
    }
}

pub struct RegBuilderWithSid {
    sid: Sid,
}

impl RegBuilderWithSid {
    pub fn with_aid<A>(self, aid: A) -> RegBuilderWithSidAid
    where
        A: IntoAid,
    {
        RegBuilderWithSidAid {
            sid: self.sid,
            aid: aid.into_aid(),
        }
    }
}

pub struct RegBuilderWithSidAid {
    sid: Sid,
    aid: Aid,
}

impl RegBuilderWithSidAid {
    pub fn with_recepient(self, recepient: WeakRecipient<DeliverMessage>) -> RegNodeMessage {
        RegNodeMessage {
            sid: self.sid,
            aid: self.aid,
            recepient,
        }
    }
}
