use actix::{Actor, Addr, Context, Handler, Message};
use nexus_counters::Counters;
use nexus_hashmap::HashmapExt;
use std::collections::HashMap;

use crate::{KeysMsg, KeysResponse, OpFailedMsg, OpSucceededMsg};

pub struct Observer {
    keys: HashMap<String, Counters<2>>,
}

impl Observer {
    fn new() -> Self {
        Self {
            keys: Default::default(),
        }
    }

    pub fn start() -> Addr<Self> {
        Observer::new().start()
    }

    fn update_succeeded(&mut self, key: &str) {
        let _ = self.keys.update_or_insert(
            key.to_string(),
            Counters::incr_succeeded,
            Counters::SUCCEEDED2,
        );
    }

    fn update_failed(&mut self, key: &str) {
        let _ =
            self.keys
                .update_or_insert(key.to_string(), Counters::incr_failed, Counters::FAILED2);
    }

    fn get_counters(&self) -> impl Iterator<Item = (String, usize, usize)> + '_ {
        self.keys.keys().map(|k| {
            let (k, totals) = self.keys.get_key_value(k).unwrap();
            (k.clone(), totals.succeeded(), totals.failed())
        })
    }
}

impl Actor for Observer {
    type Context = Context<Self>;
}

impl Handler<OpSucceededMsg> for Observer {
    type Result = <OpSucceededMsg as Message>::Result;

    fn handle(&mut self, msg: OpSucceededMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.update_succeeded(msg.op())
    }
}

impl Handler<OpFailedMsg> for Observer {
    type Result = <OpFailedMsg as Message>::Result;

    fn handle(&mut self, msg: OpFailedMsg, _ctx: &mut Self::Context) -> Self::Result {
        self.update_failed(msg.op())
    }
}

impl Handler<KeysMsg> for Observer {
    type Result = <KeysMsg as Message>::Result;

    fn handle(&mut self, _msg: KeysMsg, _ctx: &mut Self::Context) -> Self::Result {
        let tuples = self.get_counters();
        KeysResponse::new(tuples)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix::System;

    #[test]
    fn observer() {
        let sys = System::new();

        sys.block_on(async {
            let observer = Observer::start();

            // Increment success for key1
            let msg = OpSucceededMsg::new("key1");
            let _ = observer.send(msg).await.unwrap();

            let msg = OpSucceededMsg::new("key1");
            let _ = observer.send(msg).await.unwrap();

            // Increment fail for key2
            let msg = OpFailedMsg::new("key2");
            let _ = observer.send(msg).await.unwrap();

            eprintln!("Sending keys message");

            let msg = KeysMsg;
            let res = observer.send(msg).await.unwrap();

            println!("keys: {:?}", res.keys());
        });
    }
}
