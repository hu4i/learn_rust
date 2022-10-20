use tokio::{net::TcpStream, io::{AsyncWriteExt, AsyncReadExt}};
use bytes::BytesMut;

#[tokio::main]
async fn main() {
    let mut client = TcpStream::connect("127.0.0.1:6142").await.ok().unwrap();

    let mut buf = BytesMut::new();

    client.write_all(b"hello world").await.ok();
    client.read_buf(&mut buf).await.ok();

    println!("[*] Received: {:?}", buf);

}