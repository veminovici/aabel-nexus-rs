use actix::Message;
use nexus_ids::{Aid, Sid};
use nexus_message::{Header, MessageExt};

pub struct Deliver {
    from: Header,
    to: Header,
    session: Header,
}

impl Deliver {
    const SHORT_NAME: &'static str = "DLVR";

    pub fn new(fid: &Aid, tid: &Aid, sid: &Sid) -> Self {
        Self {
            from: Header::From(*fid),
            to: Header::To(*tid),
            session: Header::Session(*sid),
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

impl Message for Deliver {
    type Result = ();
}

impl MessageExt for Deliver {
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
