use nexus_message::{Deliver, Dispatch};

use crate::ActorExt;

pub trait Node: ActorExt {
    /// A short name for the node
    fn short_name(&self) -> &'static str;

    /// Handles an incoming message. It returns the collection
    /// of messages to be sent to other nodes.
    fn handle_msg(&self, _msg: Deliver) -> impl Iterator<Item = Dispatch> {
        [].into_iter()
    }
}
