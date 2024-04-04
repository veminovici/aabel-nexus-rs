use nexus_message::{Deliver, Dispatch};

pub trait Node {
    /// Handles an incoming message. It returns the collection
    /// of messages to be sent to other nodes.
    fn handle_msg(&self, msg: Deliver) -> impl Iterator<Item = Dispatch>;
}
