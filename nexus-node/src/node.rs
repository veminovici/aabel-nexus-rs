use nexus_message::{Deliver, Dispatch};

use crate::ActorExt;

pub trait Node: ActorExt {
    /// Handles an incoming message. It returns the collection
    /// of messages to be sent to other nodes.
    fn handle_msg(&self, msg: Deliver) -> impl Iterator<Item = Dispatch>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix::{Actor, Context, Handler, Message, WeakRecipient};
    use nexus_ids::Aid;
    use nexus_message::{RegKernel, RegNeighbor, UnregNeighbor};

    struct MyNode {
        krnl: Option<WeakRecipient<Dispatch>>,
        neighbors: Vec<Aid>,
    }

    impl Actor for MyNode {
        type Context = Context<Self>;
    }

    impl Handler<RegKernel> for MyNode {
        type Result = <RegKernel as Message>::Result;

        fn handle(&mut self, msg: RegKernel, _ctx: &mut Self::Context) -> Self::Result {
            self.krnl = Some(msg.recipient().clone())
        }
    }

    impl Handler<RegNeighbor> for MyNode {
        type Result = ();

        fn handle(&mut self, msg: RegNeighbor, _ctx: &mut Self::Context) -> Self::Result {
            self.neighbors.push(*msg.aid())
        }
    }

    impl Handler<UnregNeighbor> for MyNode {
        type Result = ();

        fn handle(&mut self, msg: UnregNeighbor, _ctx: &mut Self::Context) -> Self::Result {
            let aid = msg.aid();
            self.neighbors = self
                .neighbors
                .iter()
                .filter_map(|a| if a == aid { Some(*aid) } else { None })
                .collect::<Vec<_>>();
        }
    }

    impl Handler<Deliver> for MyNode {
        type Result = <Deliver as Message>::Result;

        fn handle(&mut self, msg: Deliver, _ctx: &mut Self::Context) -> Self::Result {
            for msg in self.handle_msg(msg) {
                let _ = self
                    .krnl
                    .as_ref()
                    .map(|recipient| recipient.upgrade())
                    .flatten()
                    .map(|recipient| recipient.do_send(msg));
            }
            // let _msgs = self.handle_msg(msg);
        }
    }

    impl Node for MyNode {
        fn handle_msg(&self, _msg: Deliver) -> impl Iterator<Item = Dispatch> {
            eprintln!("MyNode - is handling a message");
            [].into_iter()
        }
    }
}
