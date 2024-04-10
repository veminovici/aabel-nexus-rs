use actix::{Actor, Context, Handler, Message};
use nexus_message::{Deliver, Dispatch, RegKernel, RegNeighbor, UnregNeighbor};
use nexus_node::{BaseNode, Node};

struct PingPong(BaseNode);

impl PingPong {
    const SHORT_NAME: &'static str = "PP";
}

impl Actor for PingPong {
    type Context = Context<Self>;
}

impl Handler<RegKernel> for PingPong {
    type Result = <RegKernel as Message>::Result;

    fn handle(&mut self, msg: RegKernel, _ctx: &mut Self::Context) -> Self::Result {
        self.0.reg_kernel(msg.recipient());
    }
}

impl Handler<RegNeighbor> for PingPong {
    type Result = ();

    fn handle(&mut self, msg: RegNeighbor, _ctx: &mut Self::Context) -> Self::Result {
        self.0.reg_neighbor(msg.aid());
    }
}

impl Handler<UnregNeighbor> for PingPong {
    type Result = ();

    fn handle(&mut self, msg: UnregNeighbor, _ctx: &mut Self::Context) -> Self::Result {
        self.0.unreg_neighbor(msg.aid())
    }
}

impl Handler<Deliver> for PingPong {
    type Result = <Deliver as Message>::Result;

    fn handle(&mut self, msg: Deliver, _ctx: &mut Self::Context) -> Self::Result {
        for msg in self.handle_msg(msg) {
            self.0.do_send(msg)
        }
    }
}

impl Node for PingPong {
    fn handle_msg(&self, _msg: Deliver) -> impl Iterator<Item = Dispatch> {
        eprintln!("{}: is handling a message", Self::SHORT_NAME);
        [].into_iter()
    }

    fn short_name(&self) -> &'static str {
        Self::SHORT_NAME
    }
}
