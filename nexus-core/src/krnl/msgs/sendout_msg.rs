use crate::{Body, Fid, IntoFid, IntoSid, IntoTid, Sid, Tid};
use actix::Message;
use std::fmt::{Display, LowerHex, UpperHex};

pub struct SendOutMessage {
    sid: Sid,
    fid: Fid,
    tid: Tid,
    body: Body,
}

impl SendOutMessage {
    const MSG_ID: &'static str = "SEND";

    pub fn sid(&self) -> &Sid {
        &self.sid
    }

    pub fn fid(&self) -> &Fid {
        &self.fid
    }

    pub fn tid(&self) -> &Tid {
        &self.tid
    }

    pub fn body(&self) -> &Body {
        &self.body
    }
}

impl Message for SendOutMessage {
    type Result = ();
}

//
// Display, UpperHex, LowerHex traits
//

impl Display for SendOutMessage {
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

impl UpperHex for SendOutMessage {
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

impl LowerHex for SendOutMessage {
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
// Message builders
//

pub struct SendBuilder;

impl SendBuilder {
    pub fn with_sid<S>(sid: S) -> SendBuilderWithSid<S>
    where
        S: IntoSid,
    {
        SendBuilderWithSid { sid }
    }
}

pub struct SendBuilderWithSid<S> {
    sid: S,
}

impl<S> SendBuilderWithSid<S> {
    pub fn with_fid<F>(self, fid: F) -> SendBuilderWithSidFid<S, F>
    where
        F: IntoFid,
    {
        SendBuilderWithSidFid { sid: self.sid, fid }
    }
}

pub struct SendBuilderWithSidFid<S, F> {
    sid: S,
    fid: F,
}

impl<S, F> SendBuilderWithSidFid<S, F> {
    pub fn with_tid<T>(self, tid: T) -> SendBuilderWithSidFidTid<S, F, T>
    where
        T: IntoTid,
    {
        SendBuilderWithSidFidTid {
            sid: self.sid,
            fid: self.fid,
            tid,
        }
    }
}

pub struct SendBuilderWithSidFidTid<S, F, T> {
    sid: S,
    fid: F,
    tid: T,
}

impl<S, F, T> SendBuilderWithSidFidTid<S, F, T>
where
    S: IntoSid,
    F: IntoFid,
    T: IntoTid,
{
    pub fn with_body(self, body: impl Iterator<Item = u8>) -> SendOutMessage {
        SendOutMessage {
            sid: self.sid.into_sid(),
            fid: self.fid.into_fid(),
            tid: self.tid.into_tid(),
            body: body.into(),
        }
    }
}
