extern crate tokio_executor;
extern crate tokio_timer;

use tokio_timer::clock;
use tokio_timer::clock::*;

use std::time::Instant;
use crates_unittest::test_case;
use std::prelude::v1::*;
struct ConstNow(Instant);

impl Now for ConstNow {
    fn now(&self) -> Instant {
        self.0
    }
}

#[test_case]
fn default_clock() {
    let a = Instant::now();
    let b = clock::now();
    let c = Clock::new().now();

    assert!(a <= b);
    assert!(b <= c);
}

#[test_case]
fn custom_clock() {
    let now = ConstNow(Instant::now());
    let clock = Clock::new_with_now(now);

    let a = Instant::now();
    let b = clock.now();

    assert!(b <= a);
}

#[test_case]
fn execution_context() {
    let now = ConstNow(Instant::now());
    let clock = Clock::new_with_now(now);

    let mut enter = tokio_executor::enter().unwrap();

    with_default(&clock, &mut enter, |_| {
        let a = Instant::now();
        let b = clock::now();

        assert!(b <= a);
    });
}