use crate::{Header, MessageExt};
use actix::Message;
use nexus_ids::Aid;
use std::borrow::Borrow;

pub struct RegNeighbor(Aid);

impl Message for RegNeighbor {
    type Result = ();
}

impl MessageExt for RegNeighbor {
    fn short_name(&self) -> &str {
        "NGHB+"
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
        Self(aid)
    }
}

impl AsRef<Aid> for RegNeighbor {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl Borrow<Aid> for RegNeighbor {
    fn borrow(&self) -> &Aid {
        &self.0
    }
}
