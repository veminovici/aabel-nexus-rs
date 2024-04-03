use actix::Message;

/// A message indicating that an operation succeeded.

pub struct OpSucceededMsg(String);

impl OpSucceededMsg {
    pub fn new(op: &str) -> Self {
        Self(op.to_string())
    }

    pub fn op(&self) -> &str {
        self.0.as_str()
    }
}

impl Message for OpSucceededMsg {
    type Result = ();
}
