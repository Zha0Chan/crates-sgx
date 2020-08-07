
use futures::prelude::*;
use futures::future::{ok, select_all, err};
use std::prelude::v1::*;
use crates_unittest::test_case;

#[test_case]
fn smoke() {
    let v = vec![
        ok(1),
        err(2),
        ok(3),
    ];

    let (i, idx, v) = select_all(v).wait().ok().unwrap();
    assert_eq!(i, 1);
    assert_eq!(idx, 0);

    let (i, idx, v) = select_all(v).wait().err().unwrap();
    assert_eq!(i, 2);
    assert_eq!(idx, 0);

    let (i, idx, v) = select_all(v).wait().ok().unwrap();
    assert_eq!(i, 3);
    assert_eq!(idx, 0);

    assert!(v.is_empty());
}
