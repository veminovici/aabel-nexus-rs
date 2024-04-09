use crate::{Body, Header};
use nexus_ids::{Aid, Sid};

#[derive(Debug, Clone)]
pub struct Msg {
    from: Header,
    to: Header,
    session: Header,
    body: Body,
}

impl Msg {
    pub fn new(fid: &Aid, tid: &Aid, sid: &Sid, body: &Body) -> Self {
        Self {
            from: Header::From(*fid),
            to: Header::To(*tid),
            session: Header::Session(*sid),
            body: body.clone(),
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

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn headers(&self) -> impl Iterator<Item = &Header> {
        [&self.from, &self.to, &self.session].into_iter()
    }
}
