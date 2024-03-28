use actix::{Actor, Context, Handler, System};
use nexus_core::{Aid, DeliverMessage, Kernel, RegBuilder, SendBuilder, StatsBuilder};

struct MyActor;

impl Actor for MyActor {
    type Context = Context<Self>;
}

impl Handler<DeliverMessage> for MyActor {
    type Result = ();

    fn handle(&mut self, msg: DeliverMessage, _ctx: &mut Self::Context) -> Self::Result {
        eprintln!("RCVD: /{msg}/")
    }
}

#[actix::main]
async fn main() {
    // start new kernel
    let krnl = Kernel::new();
    let krnl_addr = krnl.start();

    // create the node
    let node_id: Aid = 500.into();
    let node = MyActor;
    let node_addr = node.start();

    // register the node
    let regnode = RegBuilder::with_sid(500)
        .with_aid(node_id)
        .with_recepient(node_addr.downgrade().recipient());
    krnl_addr.send(regnode).await.unwrap();

    // send message
    let msg = SendBuilder::with_sid(1000)
        .with_fid(10)
        .with_tid(node_id)
        .with_body([1, 2, 3].into_iter());
    krnl_addr.send(msg).await.unwrap();

    // send a second message
    let msg = SendBuilder::with_sid(1500)
        .with_fid(10)
        .with_tid(node_id)
        .with_body([1, 2, 3].into_iter());
    let _res = krnl_addr.send(msg).await;

    // Get the internal stats for the kernel.
    let msg = StatsBuilder::with_sid(2000).with_aid(15);
    let stats = krnl_addr.send(msg).await.unwrap();
    println!("NUMBERS: {stats}");

    // stop system and exit
    System::current().stop();
}
