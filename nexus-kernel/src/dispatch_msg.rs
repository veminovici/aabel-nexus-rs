use actix::Message;
use nexus_ids::{Aid, Sid};
use nexus_message::{Header, MessageExt};

pub struct Dispatch {
    from: Header,
    to: Header,
    session: Header,
}

impl Dispatch {
    const SHORT_NAME: &'static str = "DISP";

    pub fn new(fid: Aid, tid: Aid, sid: Sid) -> Self {
        Self {
            from: Header::From(fid),
            to: Header::To(tid),
            session: Header::Session(sid),
        }
    }

    pub fn fid(&self) -> &Aid {
        self.from.try_from().unwrap()
    }

    pub fn tid(&self) -> &Aid {
        self.to.try_to().unwrap()
    }

    pub fn sid(&self) -> &Sid {
        self.session.try_session().unwrap()
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
        [&self.from, &self.to, &self.session].into_iter()
    }

    fn body(&self) -> impl Iterator<Item = u8> {
        [].into_iter()
    }
}
