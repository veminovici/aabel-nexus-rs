use actix::{Actor, Context, Handler, Message};

use crate::{ObsDispatchMsg, ObsRegisterMsg};

pub struct Observer;

impl Observer {
    pub fn new() -> Self {
        Self
    }
}

impl Actor for Observer {
    type Context = Context<Self>;
}

impl Handler<ObsDispatchMsg> for Observer {
    type Result = <ObsDispatchMsg as Message>::Result;

    fn handle(&mut self, _msg: ObsDispatchMsg, _ctx: &mut Self::Context) -> Self::Result {
        eprintln!("OBS received dispatch")
    }
}

impl Handler<ObsRegisterMsg> for Observer {
    type Result = <ObsRegisterMsg as Message>::Result;

    fn handle(&mut self, _msg: ObsRegisterMsg, _ctx: &mut Self::Context) -> Self::Result {
        eprintln!("OBS received register")
    }
}
