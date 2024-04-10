use actix::{Actor, Context, Handler, Message};
use nexus_ids::{Aid, Sid};
use nexus_message::{Body, Deliver, Dispatch, RegKernel, RegNeighbor, UnregNeighbor};
use nexus_node::{BaseNode, Node};
use nexus_serialize::{from_bytes, to_bytes};

use crate::msg::PingPongMsg;

pub struct PingPong(BaseNode);

impl PingPong {
    const SHORT_NAME: &'static str = "PP";

    pub fn new(aid: Aid) -> Self {
        Self(BaseNode::new(aid))
    }

    pub fn send_ping(&self, tid: &Aid, sid: &Sid, payload: u8) {
        let body = to_bytes(&PingPongMsg::Ping(payload)).unwrap();
        let body = Body::new(body.into_iter());
        let msg = Dispatch::new(self.0.aid(), tid, sid, &body);
        self.0.do_send(msg)
    }
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
    fn handle_msg(&self, msg: Deliver) -> impl Iterator<Item = Dispatch> {
        eprintln!("{}: is handling a message", Self::SHORT_NAME);
        let body = msg.body().as_ref();
        let msgs: Vec<Dispatch> = match from_bytes(body).unwrap() {
            Some(PingPongMsg::Ping(payload)) => {
                eprintln!("Received ping [{payload}]");

                // We should reply with a pong.
                let fid = self.0.aid();
                let tid = msg.fid();
                let sid = msg.sid();
                let body = to_bytes(&PingPongMsg::Pong(payload)).unwrap();
                let body = Body::new(body.into_iter());

                let msg = Dispatch::new(fid, tid, sid, &body);
                vec![msg]
            }
            Some(PingPongMsg::Pong(v)) => {
                eprintln!("Received pong [{v}]");
                vec![]
            }
            None => {
                eprintln!("Could not deserialized the message");
                vec![]
            }
        };

        msgs.into_iter()
    }

    fn short_name(&self) -> &'static str {
        Self::SHORT_NAME
    }
}
