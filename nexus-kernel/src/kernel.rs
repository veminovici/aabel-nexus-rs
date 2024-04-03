use std::collections::HashMap;

use actix::{Actor, Context, Handler, Message, WeakRecipient};
use nexus_ids::Aid;

use crate::{Deliver, Dispatch, Register, Unregister};

pub struct Kernel {
    nodes: HashMap<Aid, WeakRecipient<Deliver>>,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
        }
    }

    fn handle_register(&mut self, aid: &Aid, recipient: &WeakRecipient<Deliver>) {
        let _ = self.nodes.insert(*aid, recipient.clone());
    }

    fn handle_unregister(&mut self, aid: &Aid) {
        let _ = self.nodes.remove(aid);
    }

    fn handle_dispatch(&self, msg: &Dispatch) {
        if let Some(recipient) = self.nodes.get(msg.tid()) {
            self.deliver_msg(msg, recipient)
        }
    }

    fn deliver_msg(&self, msg: &Dispatch, recipient: &WeakRecipient<Deliver>) {
        recipient
            .upgrade()
            .map(|recipient| {
                let fid = msg.fid();
                let tid = msg.tid();
                let sid = msg.sid();

                let dlvr = Deliver::new(fid, tid, sid);

                recipient.do_send(dlvr)
            })
            .unwrap_or(());
    }
}

impl Default for Kernel {
    fn default() -> Self {
        Self::new()
    }
}

impl Actor for Kernel {
    type Context = Context<Self>;
}

impl Handler<Register> for Kernel {
    type Result = <Register as Message>::Result;

    fn handle(&mut self, msg: Register, _ctx: &mut Self::Context) -> Self::Result {
        let aid = msg.aid();
        let recipient = msg.recipient();

        self.handle_register(aid, recipient)
    }
}

impl Handler<Dispatch> for Kernel {
    type Result = <Dispatch as Message>::Result;

    fn handle(&mut self, msg: Dispatch, _ctx: &mut Self::Context) -> Self::Result {
        self.handle_dispatch(&msg)
    }
}

impl Handler<Unregister> for Kernel {
    type Result = <Unregister as Message>::Result;

    fn handle(&mut self, msg: Unregister, _ctx: &mut Self::Context) -> Self::Result {
        let aid = msg.aid();
        self.handle_unregister(aid)
    }
}
