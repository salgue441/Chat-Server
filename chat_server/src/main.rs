// Libraries
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, address) = listener.accept().await.unwrap();

    loop {
        let mut buffer: [u8; 1024] = [0u8; 1024];
        let num_read_bytes: usize = socket.read(&mut buffer).await.unwrap();

        socket.write_all(&buffer[..num_read_bytes]).await.unwrap();
    }
}
