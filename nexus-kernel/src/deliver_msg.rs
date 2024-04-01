use crate::{Body, DispatchMsg};
use actix::Message;
use nexus_ids::{Fid, Sid, Tid};
use std::fmt::{Display, LowerHex, UpperHex};

pub struct DeliverMsg {
    sid: Sid,
    fid: Fid,
    tid: Tid,
    body: Body,
}

impl DeliverMsg {
    const SHORT_NAME: &'static str = "DLVR";
}

impl Message for DeliverMsg {
    type Result = ();
}

impl Display for DeliverMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {} : {}->{} : {}",
            Self::SHORT_NAME,
            self.sid,
            self.fid,
            self.tid,
            self.body
        )
    }
}

impl UpperHex for DeliverMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {:X} : {:X}->{:X} : {:X}",
            Self::SHORT_NAME,
            self.sid,
            self.fid,
            self.tid,
            self.body
        )
    }
}

impl LowerHex for DeliverMsg {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} : {:x} : {:x}->{:x} : {:x}",
            Self::SHORT_NAME,
            self.sid,
            self.fid,
            self.tid,
            self.body
        )
    }
}

//
// From
//

impl From<DispatchMsg> for DeliverMsg {
    fn from(dispatch_msg: DispatchMsg) -> Self {
        DeliverMsg {
            sid: dispatch_msg.sid,
            fid: dispatch_msg.fid,
            tid: dispatch_msg.tid,
            body: dispatch_msg.body,
        }
    }
}
