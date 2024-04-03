use nexus_observer::{KeysMsg, Observer, OpFailedMsg, OpSucceededMsg};

#[actix::test]
async fn observer() {
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

    eprintln!("keys: {:?}", res.keys());
}
