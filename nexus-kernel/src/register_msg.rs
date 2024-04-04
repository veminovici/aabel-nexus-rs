use actix::{Message, WeakRecipient};
use nexus_ids::Aid;
use nexus_message::{Header, MessageExt};

use crate::Deliver;

pub struct Register {
    aid: Aid,
    recipient: WeakRecipient<Deliver>,
}

impl Register {
    const SHORT_NAME: &'static str = "ACTR+";

    pub fn new(aid: Aid, recipient: WeakRecipient<Deliver>) -> Self {
        Self { aid, recipient }
    }

    pub fn aid(&self) -> &Aid {
        &self.aid
    }

    pub fn recipient(&self) -> &WeakRecipient<Deliver> {
        &self.recipient
    }
}

impl Message for Register {
    type Result = ();
}

impl MessageExt for Register {
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
