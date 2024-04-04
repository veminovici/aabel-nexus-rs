use crate::{ActorExt, Neighbors};
use actix::{Actor, Context, Handler, Message, WeakRecipient};
use nexus_ids::Aid;
use nexus_message::{Deliver, Dispatch, RegKernel, RegNeighbor, UnregNeighbor};

pub trait Node: ActorExt {
    /// Handles an incoming message. It returns the collection
    /// of messages to be sent to other nodes.
    fn handle_msg(&self, msg: Deliver) -> impl Iterator<Item = Dispatch>;
}

pub struct MyNode {
    krnl: Option<WeakRecipient<Dispatch>>,
    neighbors: Neighbors<Aid>,
}

impl MyNode {
    pub fn new() -> Self {
        Self {
            krnl: None,
            neighbors: Default::default(),
        }
    }

    fn do_send(&self, msg: Dispatch) {
        self.krnl
            .as_ref()
            .and_then(|recipient| recipient.upgrade())
            .map(|recipeint| recipeint.do_send(msg))
            .unwrap_or(())
    }
}

impl Default for MyNode {
    fn default() -> Self {
        Self::new()
    }
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
        self.neighbors += msg.aid();
    }
}

impl Handler<UnregNeighbor> for MyNode {
    type Result = ();

    fn handle(&mut self, msg: UnregNeighbor, _ctx: &mut Self::Context) -> Self::Result {
        self.neighbors -= msg.aid();
    }
}

impl Handler<Deliver> for MyNode {
    type Result = <Deliver as Message>::Result;

    fn handle(&mut self, msg: Deliver, _ctx: &mut Self::Context) -> Self::Result {
        for msg in self.handle_msg(msg) {
            self.do_send(msg)
        }
    }
}

impl Node for MyNode {
    fn handle_msg(&self, _msg: Deliver) -> impl Iterator<Item = Dispatch> {
        eprintln!("MyNode - is handling a message");
        [].into_iter()
    }
}
