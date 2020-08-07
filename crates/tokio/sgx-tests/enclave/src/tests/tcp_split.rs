#![warn(rust_2018_idioms)]
#![cfg(feature = "full")]

use std::io::Result;
use std::io::{Read, Write};
use std::{net, thread};

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use std::string::ToString;
use crates_unittest::test_case;
use tokio_test::assert_ok;
#[crates_unittest::test]
async fn split() {
    assert_ok!(split_inner().await);
}
async fn split_inner() -> std::io::Result<()> {
    const MSG: &[u8] = b"split";

    let listener = net::TcpListener::bind("127.0.0.1:0")?;
    let addr = listener.local_addr()?;

    let handle = thread::spawn(move || {
        let (mut stream, _) = listener.accept().unwrap();
        stream.write_all(MSG).unwrap();

        let mut read_buf = [0u8; 32];
        let read_len = stream.read(&mut read_buf).unwrap();
        assert_eq!(&read_buf[..read_len], MSG);
    });

    let mut stream = TcpStream::connect(&addr).await?;
    let (mut read_half, mut write_half) = stream.split();

    let mut read_buf = [0u8; 32];
    let peek_len1 = read_half.peek(&mut read_buf[..]).await?;
    let peek_len2 = read_half.peek(&mut read_buf[..]).await?;
    assert_eq!(peek_len1, peek_len2);

    let read_len = read_half.read(&mut read_buf[..]).await?;
    assert_eq!(peek_len1, read_len);
    assert_eq!(&read_buf[..read_len], MSG);

    write_half.write(MSG).await?;
    handle.join().unwrap();
    Ok(())
}