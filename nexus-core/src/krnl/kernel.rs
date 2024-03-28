use crate::{Aid, DeliverMessage, RegNodeMessage, SendOutMessage, StatsMessage, StatsResponse};
use actix::{Actor, Context, Handler, Message, WeakRecipient};
use std::collections::HashMap;

pub struct Kernel {
    stat_msg_rdvd: usize,
    nodes: HashMap<Aid, WeakRecipient<DeliverMessage>>,
}

impl Kernel {
    pub fn new() -> Self {
        Self {
            stat_msg_rdvd: 0,
            nodes: Default::default(),
        }
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

//
// Message handlers
//

impl Handler<SendOutMessage> for Kernel {
    type Result = <SendOutMessage as Message>::Result;

    fn handle(&mut self, msg: SendOutMessage, _ctx: &mut Self::Context) -> Self::Result {
        self.stat_msg_rdvd += 1;

        eprintln!("{msg}");

        let aid = msg.tid().as_ref();
        let result = self.nodes.get(aid);
        if let Some(recipient) = result {
            recipient.upgrade().map(|r| r.do_send(msg.into())).unwrap();
        }
    }
}

impl Handler<StatsMessage> for Kernel {
    type Result = <StatsMessage as Message>::Result;

    fn handle(&mut self, msg: StatsMessage, _ctx: &mut Self::Context) -> Self::Result {
        eprintln!("{msg}");
        StatsResponse::new(self.stat_msg_rdvd, self.nodes.len())
    }
}

impl Handler<RegNodeMessage> for Kernel {
    type Result = ();

    fn handle(&mut self, msg: RegNodeMessage, _ctx: &mut Self::Context) -> Self::Result {
        let aid = msg.aid();
        let recepient = msg.recepient();

        eprintln!("{msg}");

        self.nodes.insert(*aid, recepient.clone());
    }
}
