// Libraries
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

/**
 * @brief
 * Main function of the program. Handles the connection to the client and
 * reads/writes to the socket.
 */
#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel::<String>(10);

    loop {
        let (mut socket, _address) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                let bytes_read = reader.read_line(&mut line).await.unwrap();

                // If no bytes are read,
                // the client has closed the connection.
                if bytes_read == 0 {
                    break;
                }

                tx.send(line.clone()).unwrap();
                let message = rx.recv().await.unwrap();

                writer.write_all(&message.as_bytes()).await.unwrap();
                line.clear();
            }
        });
    }
}
