use crate::{Header, MessageExt};
use actix::Message;
use nexus_ids::Aid;
use std::{borrow::Borrow, fmt::Display};

pub struct UnregNeighbor(Aid);

impl UnregNeighbor {
    const SHORT_NAME: &'static str = "NGBR-";

    pub fn new(aid: Aid) -> Self {
        Self(aid)
    }

    pub fn aid(&self) -> &Aid {
        &self.0
    }
}

impl Message for UnregNeighbor {
    type Result = ();
}

impl MessageExt for UnregNeighbor {
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

impl From<Aid> for UnregNeighbor {
    fn from(aid: Aid) -> Self {
        Self(aid)
    }
}

impl AsRef<Aid> for UnregNeighbor {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl Borrow<Aid> for UnregNeighbor {
    fn borrow(&self) -> &Aid {
        &self.0
    }
}

impl Display for UnregNeighbor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}", Self::SHORT_NAME, self.0)
    }
}
