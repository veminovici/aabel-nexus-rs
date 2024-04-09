use std::borrow::Borrow;

use crate::{Body, Header, MessageExt, Msg};
use actix::Message;
use nexus_ids::{Aid, Sid};

pub struct Deliver(Msg);

impl Deliver {
    const SHORT_NAME: &'static str = "DLVR";

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

impl Message for Deliver {
    type Result = ();
}

impl MessageExt for Deliver {
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

impl AsRef<Msg> for Deliver {
    fn as_ref(&self) -> &Msg {
        &self.0
    }
}

impl Borrow<Msg> for Deliver {
    fn borrow(&self) -> &Msg {
        self.as_ref()
    }
}

impl From<&Msg> for Deliver {
    fn from(msg: &Msg) -> Self {
        Self(msg.clone())
    }
}
