// Libraries
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
};

/**
 * @brief
 * Main function of the program. Handles the connection to the client and
 * reads/writes to the socket.
 */
#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let (mut socket, address) = listener.accept().await.unwrap();

    let (reader, mut writer) = socket.split();
    let mut reader: BufReader<tokio::net::tcp::ReadHalf<'_>> = BufReader::new(reader);
    let mut line: String = String::new();

    loop {
        let bytes_read: usize = reader.read_line(&mut line).await.unwrap();

        // If no bytes are read, the client has closed the connection.
        if bytes_read == 0 {
            break;
        }

        writer.write_all(line.as_bytes()).await.unwrap();
        line.clear();
    }
}
