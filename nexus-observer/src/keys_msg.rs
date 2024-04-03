use actix::{dev::MessageResponse, Actor, Message};

/// A message that returns the stored keys and their success and fail values.
pub struct KeysMsg;

impl Message for KeysMsg {
    type Result = KeysResponse;
}

#[derive(Debug, Clone)]
pub struct KeysResponse {
    keys: Vec<(String, usize, usize)>,
}

impl KeysResponse {
    pub fn new<I>(keys: I) -> Self
    where
        I: Iterator<Item = (String, usize, usize)>,
    {
        Self {
            keys: keys.collect(),
        }
    }

    pub fn keys(&self) -> Vec<(String, usize, usize)> {
        self.keys.clone()
    }
}

impl<A> MessageResponse<A, KeysMsg> for KeysResponse
where
    A: Actor,
{
    fn handle(
        self,
        _ctx: &mut <A as Actor>::Context,
        tx: Option<actix::prelude::dev::OneshotSender<<KeysMsg as Message>::Result>>,
    ) {
        let _res = tx.map(|tx| tx.send(self)).unwrap_or_else(|| Ok(()));
    }
}
