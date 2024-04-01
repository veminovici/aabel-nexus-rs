use crate::{DeliverMsg, DispatchMsg, ObsDispatchMsg, ObsRegisterMsg, Observer, RegisterMsg};
use actix::{Actor, Addr, Context, Handler, Message, WeakRecipient};
use nexus_ids::Aid;
use std::collections::HashMap;

pub struct Kernel {
    nodes: HashMap<Aid, WeakRecipient<DeliverMsg>>,
    sidecar: Addr<Observer>,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            nodes: Default::default(),
            sidecar: Observer::new().start(),
        }
    }
}

impl Actor for Kernel {
    type Context = Context<Self>;
}

impl Handler<DispatchMsg> for Kernel {
    type Result = <DispatchMsg as Message>::Result;

    fn handle(&mut self, msg: DispatchMsg, _ctx: &mut Self::Context) -> Self::Result {
        let msg = ObsDispatchMsg::from_ok(msg.sid().clone());
        self.sidecar.do_send(msg)
    }
}

impl Handler<RegisterMsg> for Kernel {
    type Result = <RegisterMsg as Message>::Result;

    fn handle(&mut self, msg: RegisterMsg, _ctx: &mut Self::Context) -> Self::Result {
        let aid = msg.aid().clone();
        let recipient = msg.recipient().clone();
        let _res = self.nodes.insert(aid, recipient);

        let msg = ObsRegisterMsg::from_ok(msg.sid().clone(), aid);
        self.sidecar.do_send(msg)
    }
}
