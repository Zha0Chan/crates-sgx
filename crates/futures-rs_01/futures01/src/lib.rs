//! Zero-cost Futures in Rust
//!
//! This library is an implementation of futures in Rust which aims to provide
//! a robust implementation of handling asynchronous computations, ergonomic
//! composition and usage, and zero-cost abstractions over what would otherwise
//! be written by hand.
//!
//! Futures are a concept for an object which is a proxy for another value that
//! may not be ready yet. For example issuing an HTTP request may return a
//! future for the HTTP response, as it probably hasn't arrived yet. With an
//! object representing a value that will eventually be available, futures allow
//! for powerful composition of tasks through basic combinators that can perform
//! operations like chaining computations, changing the types of futures, or
//! waiting for two futures to complete at the same time.
//!
//! You can find extensive tutorials and documentations at [https://tokio.rs]
//! for both this crate (asynchronous programming in general) as well as the
//! Tokio stack to perform async I/O with.
//!
//! [https://tokio.rs]: https://tokio.rs
//!
//! ## Installation
//!
//! Add this to your `Cargo.toml`:
//!
//! ```toml
//! [dependencies]
//! futures = "0.1"
//! ```
//!
//! ## Examples
//!
//! Let's take a look at a few examples of how futures might be used:
//!
//! ```
//! extern crate futures;
//!
//! use std::io;
//! use std::time::Duration;
//! use futures::prelude::*;
//! use futures::future::Map;
//!
//! // A future is actually a trait implementation, so we can generically take a
//! // future of any integer and return back a future that will resolve to that
//! // value plus 10 more.
//! //
//! // Note here that like iterators, we're returning the `Map` combinator in
//! // the futures crate, not a boxed abstraction. This is a zero-cost
//! // construction of a future.
//! fn add_ten<F>(future: F) -> Map<F, fn(i32) -> i32>
//!     where F: Future<Item=i32>,
//! {
//!     fn add(a: i32) -> i32 { a + 10 }
//!     future.map(add)
//! }
//!
//! // Not only can we modify one future, but we can even compose them together!
//! // Here we have a function which takes two futures as input, and returns a
//! // future that will calculate the sum of their two values.
//! //
//! // Above we saw a direct return value of the `Map` combinator, but
//! // performance isn't always critical and sometimes it's more ergonomic to
//! // return a trait object like we do here. Note though that there's only one
//! // allocation here, not any for the intermediate futures.
//! fn add<'a, A, B>(a: A, b: B) -> Box<Future<Item=i32, Error=A::Error> + 'a>
//!     where A: Future<Item=i32> + 'a,
//!           B: Future<Item=i32, Error=A::Error> + 'a,
//! {
//!     Box::new(a.join(b).map(|(a, b)| a + b))
//! }
//!
//! // Futures also allow chaining computations together, starting another after
//! // the previous finishes. Here we wait for the first computation to finish,
//! // and then decide what to do depending on the result.
//! fn download_timeout(url: &str,
//!                     timeout_dur: Duration)
//!                     -> Box<Future<Item=Vec<u8>, Error=io::Error>> {
//!     use std::io;
//!     use std::net::{SocketAddr, TcpStream};
//!
//!     type IoFuture<T> = Box<Future<Item=T, Error=io::Error>>;
//!
//!     // First thing to do is we need to resolve our URL to an address. This
//!     // will likely perform a DNS lookup which may take some time.
//!     let addr = resolve(url);
//!
//!     // After we acquire the address, we next want to open up a TCP
//!     // connection.
//!     let tcp = addr.and_then(|addr| connect(&addr));
//!
//!     // After the TCP connection is established and ready to go, we're off to
//!     // the races!
//!     let data = tcp.and_then(|conn| download(conn));
//!
//!     // That all might take awhile, though, so let's not wait too long for it
//!     // to all come back. The `select` combinator here returns a future which
//!     // resolves to the first value that's ready plus the next future.
//!     //
//!     // Note we can also use the `then` combinator which is similar to
//!     // `and_then` above except that it receives the result of the
//!     // computation, not just the successful value.
//!     //
//!     // Again note that all the above calls to `and_then` and the below calls
//!     // to `map` and such require no allocations. We only ever allocate once
//!     // we hit the `Box::new()` call at the end here, which means we've built
//!     // up a relatively involved computation with only one box, and even that
//!     // was optional!
//!
//!     let data = data.map(Ok);
//!     let timeout = timeout(timeout_dur).map(Err);
//!
//!     let ret = data.select(timeout).then(|result| {
//!         match result {
//!             // One future succeeded, and it was the one which was
//!             // downloading data from the connection.
//!             Ok((Ok(data), _other_future)) => Ok(data),
//!
//!             // The timeout fired, and otherwise no error was found, so
//!             // we translate this to an error.
//!             Ok((Err(_timeout), _other_future)) => {
//!                 Err(io::Error::new(io::ErrorKind::Other, "timeout"))
//!             }
//!
//!             // A normal I/O error happened, so we pass that on through.
//!             Err((e, _other_future)) => Err(e),
//!         }
//!     });
//!     return Box::new(ret);
//!
//!     fn resolve(url: &str) -> IoFuture<SocketAddr> {
//!         // ...
//! #       panic!("unimplemented");
//!     }
//!
//!     fn connect(hostname: &SocketAddr) -> IoFuture<TcpStream> {
//!         // ...
//! #       panic!("unimplemented");
//!     }
//!
//!     fn download(stream: TcpStream) -> IoFuture<Vec<u8>> {
//!         // ...
//! #       panic!("unimplemented");
//!     }
//!
//!     fn timeout(stream: Duration) -> IoFuture<()> {
//!         // ...
//! #       panic!("unimplemented");
//!     }
//! }
//! # fn main() {}
//! ```
//!
//! Some more information can also be found in the [README] for now, but
//! otherwise feel free to jump in to the docs below!
//!
//! [README]: https://github.com/rust-lang-nursery/futures-rs#futures-rs

#![no_std]
#![doc(html_root_url = "https://docs.rs/futures/0.1")]

#![cfg_attr(all(feature = "mesalock_sgx",
                not(target_env = "sgx")), no_std)]
#![cfg_attr(all(target_env = "sgx", target_vendor = "mesalock"),
            feature(rustc_private))]
#[cfg(all(feature = "mesalock_sgx", not(target_env = "sgx")))]
#[macro_use]
extern crate sgx_tstd as std;


extern crate futures;

#[doc(inline)]
pub use futures::*;
