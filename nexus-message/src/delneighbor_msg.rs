use actix::Message;
use nexus_ids::Aid;
use nexus_message::{Header, MessageExt};
use std::borrow::Borrow;

pub struct DelNeighbor(Aid);

impl Message for DelNeighbor {
    type Result = ();
}

impl MessageExt for DelNeighbor {
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

impl From<Aid> for DelNeighbor {
    fn from(aid: Aid) -> Self {
        Self(aid)
    }
}

impl AsRef<Aid> for DelNeighbor {
    fn as_ref(&self) -> &Aid {
        &self.0
    }
}

impl Borrow<Aid> for DelNeighbor {
    fn borrow(&self) -> &Aid {
        &self.0
    }
}
