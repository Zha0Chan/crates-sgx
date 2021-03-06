#![doc(html_root_url = "https://docs.rs/tokio-tcp/0.1.3")]
#![deny(missing_docs, missing_debug_implementations)]

//! TCP bindings for `tokio`.
//!
//! > **Note:** This crate is **deprecated in tokio 0.2.x** and has been moved
//! > into [`tokio::net`] behind the `tcp` [feature flag].
//!
//! [`tokio::net`]: https://docs.rs/tokio/latest/tokio/net/index.html
//! [feature flag]: https://docs.rs/tokio/latest/tokio/index.html#feature-flags
//!
//! This module contains the TCP networking types, similar to the standard
//! library, which can be used to implement networking protocols.
//!
//! Connecting to an address, via TCP, can be done using [`TcpStream`]'s
//! [`connect`] method, which returns [`ConnectFuture`]. `ConnectFuture`
//! implements a future which returns a `TcpStream`.
//!
//! To listen on an address [`TcpListener`] can be used. `TcpListener`'s
//! [`incoming`][incoming_method] method can be used to accept new connections.
//! It return the [`Incoming`] struct, which implements a stream which returns
//! `TcpStream`s.
//!
//! [`TcpStream`]: struct.TcpStream.html
//! [`connect`]: struct.TcpStream.html#method.connect
//! [`ConnectFuture`]: struct.ConnectFuture.html
//! [`TcpListener`]: struct.TcpListener.html
//! [incoming_method]: struct.TcpListener.html#method.incoming
//! [`Incoming`]: struct.Incoming.html
#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;

extern crate bytes;
#[macro_use]
extern crate futures;
extern crate iovec;
extern crate mio;
extern crate tokio_io;
extern crate tokio_reactor;

mod incoming;
mod listener;
mod stream;

pub use self::incoming::Incoming;
pub use self::listener::TcpListener;
pub use self::stream::ConnectFuture;
pub use self::stream::TcpStream;
