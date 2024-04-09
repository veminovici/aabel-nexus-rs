use std::borrow::Borrow;

use crate::{Body, Header, MessageExt, Msg};
use actix::Message;
use nexus_ids::{Aid, Sid};

pub struct Dispatch(Msg);

impl Dispatch {
    const SHORT_NAME: &'static str = "DISP";

    pub fn new(fid: &Aid, tid: &Aid, sid: &Sid, body: &Body) -> Self {
        Self(Msg::new(fid, tid, sid, body))
    }

    pub fn fid(&self) -> &Aid {
        self.0.fid()
    }

    pub fn tid(&self) -> &Aid {
        self.0.tid()
    }

    pub fn sid(&self) -> &Sid {
        self.0.sid()
    }

    pub fn body(&self) -> &Body {
        self.0.body()
    }
}

impl Message for Dispatch {
    type Result = ();
}

impl MessageExt for Dispatch {
    fn short_name(&self) -> &str {
        Self::SHORT_NAME
    }

    fn headers(&self) -> impl Iterator<Item = &Header> {
        self.0.headers()
    }

    fn body(&self) -> impl Iterator<Item = u8> {
        self.0.body().as_ref().iter().copied()
    }
}

impl AsRef<Msg> for Dispatch {
    fn as_ref(&self) -> &Msg {
        &self.0
    }
}

impl Borrow<Msg> for Dispatch {
    fn borrow(&self) -> &Msg {
        self.as_ref()
    }
}

impl From<&Msg> for Dispatch {
    fn from(msg: &Msg) -> Self {
        Self(msg.clone())
    }
}
