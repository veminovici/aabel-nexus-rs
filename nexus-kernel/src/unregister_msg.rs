use actix::Message;
use nexus_ids::Aid;
use nexus_message::{Header, MessageExt};

pub struct Unregister(Aid);

impl Unregister {
    const SHORT_NAME: &'static str = "UNREG";

    pub fn new(aid: Aid) -> Self {
        Self(aid)
    }

    pub fn aid(&self) -> &Aid {
        &self.0
    }
}

impl Message for Unregister {
    type Result = ();
}

impl MessageExt for Unregister {
    fn short_name(&self) -> &str {
        Self::SHORT_NAME
    }

    fn headers(&self) -> impl Iterator<Item = &Header> {
        [].into_iter()
    }

    fn body(&self) -> impl Iterator<Item = u8> {
        [].into_iter()
    }
}