use std::time::SystemTime;

use actix::{Actor, System};
use nexus_kernel::{DispatchMsg, Kernel};

fn main() {
    let sys = System::new();
    let now = SystemTime::now();

    sys.block_on(async {
        println!("Setting up kernel");

        let kernel = Kernel::new().start();

        println!("Sending a dispatch message");

        let msg = DispatchMsg::with_sid(10.into())
            .with_fid(100.into())
            .with_tid(200.into())
            .with_body([1, 2, 3].into_iter())
            .build();
        let _ = kernel.send(msg).await.unwrap();

        println!("Test is done!");
    });

    // sys.run().unwrap();

    match now.elapsed() {
        Ok(elapsed) => println!(
            "Time taken: {}.{:06} seconds",
            elapsed.as_secs(),
            elapsed.subsec_micros()
        ),
        Err(e) => println!("An error occurred: {:?}", e),
    }
}
