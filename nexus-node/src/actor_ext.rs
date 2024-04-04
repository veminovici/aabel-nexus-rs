use actix::{Actor, Handler};
use nexus_message::{Deliver, RegKernel, RegNeighbor, UnregNeighbor};

pub trait ActorExt: Actor
where
    Self: Handler<Deliver>,
    Self: Handler<RegNeighbor>,
    Self: Handler<UnregNeighbor>,
    Self: Handler<RegKernel>,
{
}

impl<T> ActorExt for T
where
    T: Handler<Deliver>,
    T: Handler<RegNeighbor>,
    T: Handler<UnregNeighbor>,
    T: Handler<RegKernel>,
{
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix::{Context, Message};

    struct MyActor;

    impl Actor for MyActor {
        type Context = Context<Self>;
    }

    impl Handler<RegNeighbor> for MyActor {
        type Result = ();

        fn handle(&mut self, _msg: RegNeighbor, _ctx: &mut Self::Context) -> Self::Result {
            todo!()
        }
    }

    impl Handler<UnregNeighbor> for MyActor {
        type Result = ();

        fn handle(&mut self, _msg: UnregNeighbor, _ctx: &mut Self::Context) -> Self::Result {
            todo!()
        }
    }

    impl Handler<Deliver> for MyActor {
        type Result = <Deliver as Message>::Result;

        fn handle(&mut self, _msg: Deliver, _ctx: &mut Self::Context) -> Self::Result {
            todo!()
            // let _msgs = self.handle_msg(msg);
        }
    }

    impl Handler<RegKernel> for MyActor {
        type Result = <RegKernel as Message>::Result;

        fn handle(&mut self, _msg: RegKernel, _ctx: &mut Self::Context) -> Self::Result {
            todo!()
        }
    }
}
