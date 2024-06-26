use crate::Deliver;
use crate::{Header, MessageExt};
use actix::{Message, WeakRecipient};
use nexus_ids::Aid;
use std::fmt::Display;

pub struct RegActor {
    aid: Aid,
    recipient: WeakRecipient<Deliver>,
}

impl RegActor {
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

impl Message for RegActor {
    type Result = ();
}

impl MessageExt for RegActor {
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

impl Display for RegActor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", Self::SHORT_NAME, self.aid)
    }
}
