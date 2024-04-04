# Simplee > Aabel > Nexus

[![CI][ci-badge]][ci-url]
![GitHub top language][lang-badge]
[![License:MIT][license-badge]][license-url]
![GitHub code size in bytes][size-badge]
![GitHub last commit][last-commit-badge]
![GitHub watchers][watchers-badge]

A set of crates for distributed algorithms. They use an actor model and the [actix][actix_url] actor framework.

## Crates
- The [nexus-ids][nexus-ids-folder] crate implements the actor and session identifiers.
- The [nexus-hashmap][nexus-hashmap-folder] crate implements several extensions for the HashMap.
- The [nexus-list][nexus-list-folder] crate implements an extension to list, by implementing Add, AddAssign, Sub, and SubAssign traits.
- The [nexus-lattice][nexus-lattice-folder] crate defines *Join* and *Meet* traits, basic operations in the [lattice](https://en.wikipedia.org/wiki/Join_and_meet) theory.
- The [nexus-counters][nexus-counters-folder] crate implements a distributed counter. In our case it will be used to increment different values.
- The [nexus-session-store][nexus-session-store-folder] crate implements a store for session. Each session can store values for each node.
- The [nexus-message][nexus-message-folder] crate defines the *MessageExt* trait, an extention for the *actix::Message* trait. The crate also defines the kernel and node control messages.
- The [nexus-observer][nexus-observer-folder] crate implements the **Observer** actor. The actor counts the number of successful and failed operations during the execution of the run.
- The [nexus-kernel][nexus-kernel-folder] crate implements the **Kernel** actor. The actor receives and delivers messages to other actors.
- The [nexus-node][nexus-node-folder] crate defines the *ActorExt* trait, an extention for the actix::Actor trait.

## Examples
You can find an example of how to use the Observer actor at in the [test_observer](./nexus-observer//tests/test_observer.rs) file.

```rust
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
```

A second example show how to use the kernel actor, which you can find it in the [test_kernel](./nexus-kernel/tests/test_kernel.rs) file.

```rust
let kernel = Kernel::new().start();
let actor = MyActor::new().start();

// Register the actor (it will be the destination of a message)
let aid = 2.into();
let msg = Register::new(aid, actor.downgrade().recipient());
let _ = kernel.send(msg).await.unwrap();

// Dispatch a message to the actor we just created
let fid = Aid::from(1);
let tid = aid; // this is the aid of the actor we just created
let sid = Sid::from(fid);
let msg = Dispatch::new(fid, tid, sid);
let _ = kernel.send(msg).await.unwrap();

// Validate that the actor received and processed the message
let valid = actor.send(Validate).await.unwrap();
assert!(valid);

// Unregister the actor
let msg = Unregister::new(aid);
let _ = kernel.send(msg).await.unwrap();
```

## About
> Code designed and written on the beautiful island of [**Saaremaa**][url_estonia], Estonia.

[crates-url]: https://crates.io/crates/aabel-nexus-rs
[ci-badge]: https://github.com/veminovici/aabel-nexus-rs/actions/workflows/ci.yml/badge.svg?branch=main
[ci-url]: https://github.com/veminovici/aabel-nexus-rs/actions/workflows/ci.yml
[lang-badge]: https://img.shields.io/github/languages/top/veminovici/aabel-nexus-rs
[license-badge]: https://img.shields.io/badge/License-MIT-yellow.svg
[license-url]: https://opensource.org/licenses/MIT
[size-badge]: https://img.shields.io/github/languages/code-size/veminovici/aabel-nexus-rs
[last-commit-badge]: https://img.shields.io/github/last-commit/veminovici/aabel-nexus-rs
[watchers-badge]: https://img.shields.io/github/watchers/veminovici/aabel-nexus-rs
[url_estonia]: https://goo.gl/maps/DmB9ewY2R3sPGFnTA
[nexus-ids-folder]: ./nexus-ids/
[nexus-hashmap-folder]: ./nexus-hashmap/
[nexus-list-folder]: ./nexus-list/
[nexus-lattice-folder]: ./nexus-lattice/
[nexus-counters-folder]: ./nexus-counters/
[nexus-message-folder]: ./nexus-message/
[nexus-session-store-folder]: ./nexus-session-store/
[nexus-observer-folder]: ./nexus-observer/
[nexus-kernel-folder]: ./nexus-kernel/
[nexus-node-folder]: ./nexus-node/
[def]: ./nexus-hashmap/
[actix_url]: https://actix.rs/docs/actix