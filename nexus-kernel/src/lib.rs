//!
//! # Example
//!
//! ```
//! use actix::{Actor, Context, Handler, Message};
//! use nexus_ids::{Aid, Sid};
//! use nexus_kernel::Kernel;
//! use nexus_message::{Body, Deliver, Dispatch, RegActor, UnregActor};
//!
//! struct Validate;
//!
//! impl Message for Validate {
//!     type Result = bool;
//! }
//!
//! struct MyActor {
//!     rcvd: bool,
//! }
//!
//! impl MyActor {
//!     pub fn new() -> Self {
//!         Self { rcvd: false }
//!     }
//! }
//!
//! impl Actor for MyActor {
//!     type Context = Context<Self>;
//! }
//!
//! impl Handler<Deliver> for MyActor {
//!     type Result = <Deliver as Message>::Result;
//!
//!     fn handle(&mut self, _msg: Deliver, _ctx: &mut Self::Context) -> Self::Result {
//!         eprintln!("Actor received the message");
//!         self.rcvd = true;
//!     }
//! }
//!
//! impl Handler<Validate> for MyActor {
//!     type Result = <Validate as Message>::Result;
//!
//!     fn handle(&mut self, _msg: Validate, _ctx: &mut Self::Context) -> Self::Result {
//!         self.rcvd
//!     }
//! }
//!
//! async fn test() {
//!   let kernel = Kernel::new().start();
//!   let actor = MyActor::new().start();
//!
//!   // Register the actor
//!   let aid = 2.into();
//!   let msg = RegActor::new(aid, actor.downgrade().recipient());
//!   let _ = kernel.send(msg).await.unwrap();
//!
//!   // Dispatch a message.
//!   let fid = Aid::from(1);
//!   let tid = aid;
//!   let sid = Sid::from(fid);
//!   let body = Body::new([].into_iter());
//!   let msg = Dispatch::new(&fid, &tid, &sid, &body);
//!   let _ = kernel.send(msg).await.unwrap();
//!
//!   // Validate
//!   let valid = actor.send(Validate).await.unwrap();
//!   assert!(valid);
//!
//!   // Unregister the actor
//!   let msg = UnregActor::from(aid);
//!   let _ = kernel.send(msg).await.unwrap();
//! }
//! ```

mod kernel;

pub use kernel::*;
