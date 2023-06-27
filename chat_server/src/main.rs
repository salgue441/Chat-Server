/**
 * @file main.rs
 * @brief Main file for the chat server.
 * @author Carlos Salguero
 * @version 1.0
 * @date 2023-06-26
 *
 * @copyright Copyright (c) - MIT License
 */
// Libraries
use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

/**
 * @brief
 * Main function of the program. Handles the connection to the client and
 * reads/writes to the socket.
 * @details Uses the tokio library to handle the asynchronous nature of the
 * program.
 */
#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:8080").await.unwrap();
    let (tx, _rx) = broadcast::channel(10);

    loop {
        let (mut socket, address) = listener.accept().await.unwrap();

        let tx = tx.clone();
        let mut rx = tx.subscribe();

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                tokio::select! {
                    result = reader.read_line(&mut line) => {
                        if result.unwrap() == 0 {
                            break;
                        }


                        tx.send((line.clone(), address)).unwrap();
                        line.clear();
                    }

                    // If there is a message to send, send it to the client
                    result = rx.recv() => {
                        let (message, other_address) = result.unwrap();

                        if address != other_address {
                            writer.write_all(message.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
