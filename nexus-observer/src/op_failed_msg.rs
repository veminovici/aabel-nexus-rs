use actix::Message;

/// A message indicating that an operation failed.
pub struct OpFailedMsg(String);

impl OpFailedMsg {
    pub fn new(op: &str) -> Self {
        Self(op.to_string())
    }

    pub fn op(&self) -> &str {
        self.0.as_str()
    }
}

impl Message for OpFailedMsg {
    type Result = ();
}
