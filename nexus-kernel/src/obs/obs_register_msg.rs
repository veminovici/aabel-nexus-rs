use actix::Message;
use nexus_ids::{Aid, Sid};

pub struct ObsRegisterMsg {
    sid: Sid,
    aid: Aid,
    res: Result<(), ()>,
}

impl Message for ObsRegisterMsg {
    type Result = ();
}

impl ObsRegisterMsg {
    pub fn from_ok(sid: Sid, aid: Aid) -> Self {
        Self {
            sid,
            aid,
            res: Ok(()),
        }
    }

    pub fn from_err(sid: Sid, aid: Aid) -> Self {
        Self {
            sid,
            aid,
            res: Err(()),
        }
    }
}
