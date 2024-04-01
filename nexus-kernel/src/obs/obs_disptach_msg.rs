use actix::Message;
use nexus_ids::Sid;

pub struct ObsDispatchMsg {
    sid: Sid,
    res: Result<(), ()>,
}

impl Message for ObsDispatchMsg {
    type Result = ();
}

impl ObsDispatchMsg {
    pub fn from_ok(sid: Sid) -> Self {
        Self { sid, res: Ok(()) }
    }

    pub fn from_err(sid: Sid) -> Self {
        Self { sid, res: Err(()) }
    }
}
