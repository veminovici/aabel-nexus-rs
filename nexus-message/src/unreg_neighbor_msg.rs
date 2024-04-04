use crate::{Header, MessageExt};
use actix::Message;
use nexus_ids::Aid;
use std::borrow::Borrow;

pub struct UnregNeighbor(Aid);

impl Message for UnregNeighbor {
    type Result = ();
}

impl MessageExt for UnregNeighbor {
    fn short_name(&self) -> &str {
        "NGHB-"
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
