#![feature(conservative_impl_trait)]

#[macro_use]
extern crate log;
#[macro_use]
extern crate bitflags;
//#[macro_use]
//extern crate lazy_static;

extern crate bytes;
#[macro_use]
extern crate futures;
#[macro_use]
extern crate tokio_io;
extern crate tokio_core;

mod actor;
mod arbiter;
mod address;
mod sync_address;
mod builder;
mod context;
mod message;
mod sink;
mod framed;
mod system;
mod utils;

pub mod fut;
pub mod prelude;

pub use actor::{Actor, MessageHandler, StreamHandler};
pub use address::{Address, SyncAddress, Subscriber, AsyncSubscriber};
pub use arbiter::{Arbiter, StopArbiter, ArbiterAddress};
pub use builder::ServiceBuilder;
pub use context::Context;
pub use framed::{CtxFramed, CtxFramedRead, CtxFramedWrite};
pub use message::{MessageResult, MessageFuture, MessageFutureResult, MessageFutureError};
pub use sink::Sink;
pub use system::{System, SystemExit};
pub use utils::Condition;
