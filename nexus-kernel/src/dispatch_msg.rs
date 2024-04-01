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
    const SHORT_NAME: &'static str = "DISP";
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

pub struct BuilderDispatchMsg;

impl BuilderDispatchMsg {
    pub fn with_sid(sid: Sid) -> BuilderWithSid {
        BuilderWithSid { sid }
    }
}

pub struct BuilderWithSid {
    sid: Sid,
}

impl From<Sid> for BuilderWithSid {
    fn from(sid: Sid) -> Self {
        Self { sid }
    }
}

impl BuilderWithSid {
    pub fn with_fid(self, fid: Fid) -> BuilderWithSidFid {
        BuilderWithSidFid { sid: self.sid, fid }
    }
}

pub struct BuilderWithSidFid {
    sid: Sid,
    fid: Fid,
}

impl BuilderWithSidFid {
    pub fn with_tid(self, tid: Tid) -> BuilderWithSidFidTid {
        BuilderWithSidFidTid {
            sid: self.sid,
            fid: self.fid,
            tid,
        }
    }
}

pub struct BuilderWithSidFidTid {
    sid: Sid,
    fid: Fid,
    tid: Tid,
}

impl BuilderWithSidFidTid {
    pub fn with_body<I>(self, values: I) -> BuilderComplete
    where
        I: Iterator<Item = u8>,
    {
        BuilderComplete {
            sid: self.sid,
            fid: self.fid,
            tid: self.tid,
            body: values.into(),
        }
    }
}

pub struct BuilderComplete {
    sid: Sid,
    fid: Fid,
    tid: Tid,
    body: Body,
}

impl BuilderComplete {
    pub fn build(self) -> DispatchMsg {
        DispatchMsg {
            sid: self.sid,
            fid: self.fid,
            tid: self.tid,
            body: self.body,
        }
    }
}
