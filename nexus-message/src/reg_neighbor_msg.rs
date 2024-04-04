use crate::{Header, MessageExt};
use actix::Message;
use nexus_ids::Aid;
use std::{borrow::Borrow, fmt::Display};

pub struct RegNeighbor(Aid);

impl RegNeighbor {
    const SHORT_NAME: &'static str = "NGBR+";

    pub fn new(aid: Aid) -> Self {
        Self(aid)
    }

    pub fn aid(&self) -> &Aid {
        &self.0
    }
}

impl Message for RegNeighbor {
    type Result = ();
}

impl MessageExt for RegNeighbor {
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

impl From<Aid> for RegNeighbor {
    fn from(aid: Aid) -> Self {
        Self::new(aid)
    }
}

impl AsRef<Aid> for RegNeighbor {
    fn as_ref(&self) -> &Aid {
        self.aid()
    }
}

impl Borrow<Aid> for RegNeighbor {
    fn borrow(&self) -> &Aid {
        self.as_ref()
    }
}

impl Display for RegNeighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", Self::SHORT_NAME, self.0)
    }
}
