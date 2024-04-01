use crate::Body;
use actix::Message;
use nexus_ids::{Fid, Sid, Tid};
use std::fmt::{Display, LowerHex, UpperHex};

pub struct DispatchMsg {
    pub(crate) sid: Sid,
    pub(crate) fid: Fid,
    pub(crate) tid: Tid,
    pub(crate) body: Body,
}

impl DispatchMsg {
    pub(crate) const SHORT_NAME: &'static str = "DISP";

    pub fn with_sid(sid: Sid) -> BuildDispatchMsgWithSid {
        BuildDispatchMsgWithSid { sid }
    }

    pub fn sid(&self) -> &Sid {
        &self.sid
    }
}

impl Message for DispatchMsg {
    type Result = ();
}

impl Display for DispatchMsg {
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

impl UpperHex for DispatchMsg {
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

impl LowerHex for DispatchMsg {
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
// Builder pattern
//

pub struct BuildDispatchMsgWithSid {
    sid: Sid,
}

impl From<Sid> for BuildDispatchMsgWithSid {
    fn from(sid: Sid) -> Self {
        Self { sid }
    }
}

impl BuildDispatchMsgWithSid {
    pub fn with_fid(self, fid: Fid) -> BuildDispatchMsgWithSidFid {
        BuildDispatchMsgWithSidFid { sid: self.sid, fid }
    }
}

pub struct BuildDispatchMsgWithSidFid {
    sid: Sid,
    fid: Fid,
}

impl BuildDispatchMsgWithSidFid {
    pub fn with_tid(self, tid: Tid) -> BuildDispatchMsgWithSidFidTid {
        BuildDispatchMsgWithSidFidTid {
            sid: self.sid,
            fid: self.fid,
            tid,
        }
    }
}

pub struct BuildDispatchMsgWithSidFidTid {
    sid: Sid,
    fid: Fid,
    tid: Tid,
}

impl BuildDispatchMsgWithSidFidTid {
    pub fn with_body<I>(self, values: I) -> BuildDispatchMsgComplete
    where
        I: Iterator<Item = u8>,
    {
        BuildDispatchMsgComplete {
            sid: self.sid,
            fid: self.fid,
            tid: self.tid,
            body: values.into(),
        }
    }
}

pub struct BuildDispatchMsgComplete {
    sid: Sid,
    fid: Fid,
    tid: Tid,
    body: Body,
}

impl BuildDispatchMsgComplete {
    pub fn build(self) -> DispatchMsg {
        DispatchMsg {
            sid: self.sid,
            fid: self.fid,
            tid: self.tid,
            body: self.body,
        }
    }
}
