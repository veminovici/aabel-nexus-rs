use crate::{Body, Fid, SendOutMessage, Sid, Tid};
use actix::Message;
use std::fmt::{Display, LowerHex, UpperHex};

pub struct DeliverMessage {
    sid: Sid,
    fid: Fid,
    tid: Tid,
    body: Body,
}

impl DeliverMessage {
    const MSG_ID: &'static str = "DLVR";

    pub fn sid(&self) -> &Sid {
        &self.sid
    }

    pub fn fid(&self) -> &Fid {
        &self.fid
    }

    pub fn tid(&self) -> &Tid {
        &self.tid
    }
}

impl Message for DeliverMessage {
    type Result = ();
}

//
// Display, UpperHex, LowerHex traits
//

impl Display for DeliverMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ▫︎ {} ▫︎ {}->{} ▫︎ body=[{}]",
            Self::MSG_ID,
            self.sid,
            self.fid,
            self.tid,
            self.body
        )
    }
}

impl UpperHex for DeliverMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ▫︎ {} ▫︎ {}->{} ▫︎ body=[{:X}]",
            Self::MSG_ID,
            self.sid,
            self.fid,
            self.tid,
            self.body
        )
    }
}

impl LowerHex for DeliverMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} ▫︎ {} ▫︎ {}->{} ▫︎ body=[{:x}]",
            Self::MSG_ID,
            self.sid,
            self.fid,
            self.tid,
            self.body
        )
    }
}

//
// From traits
//

impl From<SendOutMessage> for DeliverMessage {
    fn from(msg: SendOutMessage) -> Self {
        Self {
            sid: *msg.sid(),
            fid: *msg.fid(),
            tid: *msg.tid(),
            body: msg.body().clone(),
        }
    }
}
