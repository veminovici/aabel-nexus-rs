use actix::{Actor, Context, Handler, Message};
use nexus_message::{Deliver, Dispatch, RegKernel, RegNeighbor, UnregNeighbor};
use nexus_node::{BaseNode, Node};

pub struct MyNode(BaseNode);

impl MyNode {
    const SHORT_NAME: &'static str = "MYND";

    pub fn new() -> Self {
        Self(Default::default())
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
        self.0.reg_kernel(msg.recipient());
    }
}

impl Handler<RegNeighbor> for MyNode {
    type Result = ();

    fn handle(&mut self, msg: RegNeighbor, _ctx: &mut Self::Context) -> Self::Result {
        self.0.reg_neighbor(msg.aid());
    }
}

impl Handler<UnregNeighbor> for MyNode {
    type Result = ();

    fn handle(&mut self, msg: UnregNeighbor, _ctx: &mut Self::Context) -> Self::Result {
        self.0.unreg_neighbor(msg.aid());
    }
}

impl Handler<Deliver> for MyNode {
    type Result = <Deliver as Message>::Result;

    fn handle(&mut self, msg: Deliver, _ctx: &mut Self::Context) -> Self::Result {
        for msg in self.handle_msg(msg) {
            self.0.do_send(msg)
        }
    }
}

impl Node for MyNode {
    fn handle_msg(&self, _msg: Deliver) -> impl Iterator<Item = Dispatch> {
        eprintln!("{}: is handling a message", Self::SHORT_NAME);
        [].into_iter()
    }

    fn short_name(&self) -> &'static str {
        Self::SHORT_NAME
    }
}

fn main() {}
