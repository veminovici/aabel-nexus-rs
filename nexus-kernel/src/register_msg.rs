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
    pub(crate) const SHORT_NAME: &'static str = "REGN";

    pub fn sid(&self) -> &Sid {
        &self.sid
    }

    pub fn aid(&self) -> &Aid {
        &self.aid
    }

    pub fn recipient(&self) -> &WeakRecipient<DeliverMsg> {
        &self.recipient
    }
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

pub struct BuildRegisterMsg;

impl BuildRegisterMsg {
    pub fn with_sid(sid: Sid) -> BuildRegisterMsgWithSid {
        BuildRegisterMsgWithSid { sid }
    }
}

pub struct BuildRegisterMsgWithSid {
    sid: Sid,
}

impl BuildRegisterMsgWithSid {
    pub fn with_aid(self, aid: Aid) -> BuildRegisterMsgWithSidAid {
        BuildRegisterMsgWithSidAid { sid: self.sid, aid }
    }
}

pub struct BuildRegisterMsgWithSidAid {
    sid: Sid,
    aid: Aid,
}

impl BuildRegisterMsgWithSidAid {
    pub fn with_ricepient(self, recipient: WeakRecipient<DeliverMsg>) -> BuildRegisterMsgComplete {
        BuildRegisterMsgComplete {
            sid: self.sid,
            aid: self.aid,
            recipient,
        }
    }
}

pub struct BuildRegisterMsgComplete {
    sid: Sid,
    aid: Aid,
    recipient: WeakRecipient<DeliverMsg>,
}

impl BuildRegisterMsgComplete {
    pub fn build(self) -> RegisterMsg {
        RegisterMsg {
            sid: self.sid,
            aid: self.aid,
            recipient: self.recipient,
        }
    }
}
