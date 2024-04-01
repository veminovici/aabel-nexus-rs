use actix::{Message, WeakRecipient};
use nexus_ids::{Aid, Sid};
use std::fmt::{Display, LowerHex, UpperHex};

use crate::DeliverMsg;

pub struct RegisterMsg {
    sid: Sid,
    aid: Aid,
    recipient: WeakRecipient<DeliverMsg>,
}

impl RegisterMsg {
    const SHORT_NAME: &'static str = "REGN";
}

impl Message for RegisterMsg {
    type Result = ();
}

impl Display for RegisterMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {} : {}", Self::SHORT_NAME, self.sid, self.aid,)
    }
}

impl UpperHex for RegisterMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:X} : {:X}", Self::SHORT_NAME, self.sid, self.aid,)
    }
}

impl LowerHex for RegisterMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} : {:x} : {:x}", Self::SHORT_NAME, self.sid, self.aid,)
    }
}

//
// Builder pattern
//

pub struct BuilderRegisterMsg;

impl BuilderRegisterMsg {
    pub fn with_sid(sid: Sid) -> BuilderWithSid {
        BuilderWithSid { sid }
    }
}

pub struct BuilderWithSid {
    sid: Sid,
}

impl BuilderWithSid {
    pub fn with_aid(self, aid: Aid) -> BuilderWithSidAid {
        BuilderWithSidAid { sid: self.sid, aid }
    }
}

pub struct BuilderWithSidAid {
    sid: Sid,
    aid: Aid,
}

impl BuilderWithSidAid {
    pub fn with_ricepient(self, recipient: WeakRecipient<DeliverMsg>) -> BuilderComplete {
        BuilderComplete {
            sid: self.sid,
            aid: self.aid,
            recipient,
        }
    }
}

pub struct BuilderComplete {
    sid: Sid,
    aid: Aid,
    recipient: WeakRecipient<DeliverMsg>,
}

impl BuilderComplete {
    pub fn build(self) -> RegisterMsg {
        RegisterMsg {
            sid: self.sid,
            aid: self.aid,
            recipient: self.recipient,
        }
    }
}
