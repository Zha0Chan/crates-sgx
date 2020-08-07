#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use tokio::sync::oneshot;
use tokio::time::{self, timeout, timeout_at, Instant};
use tokio_test::*;

use futures::future::pending;
use std::time::Duration;
use std::string::ToString;
use crates_unittest::test_case;

#[crates_unittest::test]
async fn simultaneous_deadline_future_completion() {
    // Create a future that is immediately ready
    let mut fut = task::spawn(timeout_at(Instant::now(), async {}));

    // Ready!
    assert_ready_ok!(fut.poll());
}

#[crates_unittest::test]
async fn completed_future_past_deadline() {
    // Wrap it with a deadline
    let mut fut = task::spawn(timeout_at(Instant::now() - ms(1000), async {}));

    // Ready!
    assert_ready_ok!(fut.poll());
}

#[crates_unittest::test]
async fn future_and_deadline_in_future() {
    time::pause();

    // Not yet complete
    let (tx, rx) = oneshot::channel();

    // Wrap it with a deadline
    let mut fut = task::spawn(timeout_at(Instant::now() + ms(100), rx));

    assert_pending!(fut.poll());

    // Turn the timer, it runs for the elapsed time
    time::advance(ms(90)).await;

    assert_pending!(fut.poll());

    // Complete the future
    tx.send(()).unwrap();
    assert!(fut.is_woken());

    assert_ready_ok!(fut.poll()).unwrap();
}

#[crates_unittest::test]
async fn future_and_timeout_in_future() {
    time::pause();

    // Not yet complete
    let (tx, rx) = oneshot::channel();

    // Wrap it with a deadline
    let mut fut = task::spawn(timeout(ms(100), rx));

    // Ready!
    assert_pending!(fut.poll());

    // Turn the timer, it runs for the elapsed time
    time::advance(ms(90)).await;

    assert_pending!(fut.poll());

    // Complete the future
    tx.send(()).unwrap();

    assert_ready_ok!(fut.poll()).unwrap();
}

#[crates_unittest::test]
async fn deadline_now_elapses() {
    use futures::future::pending;

    time::pause();

    // Wrap it with a deadline
    let mut fut = task::spawn(timeout_at(Instant::now(), pending::<()>()));

    // Factor in jitter
    // TODO: don't require this
    time::advance(ms(1)).await;

    assert_ready_err!(fut.poll());
}

#[crates_unittest::test]
async fn deadline_future_elapses() {
    time::pause();

    // Wrap it with a deadline
    let mut fut = task::spawn(timeout_at(Instant::now() + ms(300), pending::<()>()));

    assert_pending!(fut.poll());

    time::advance(ms(301)).await;

    assert!(fut.is_woken());
    assert_ready_err!(fut.poll());
}

fn ms(n: u64) -> Duration {
    Duration::from_millis(n)
}