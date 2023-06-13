use std::net::SocketAddr;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpListener,
};

#[tokio::main]
async fn main() {
    let listener: TcpListener = TcpListener::bind("localhost:7878").await.unwrap();

    let (mut socket, address) = listener.accept().await.unwrap();

    loop {
        let mut input_buffer = [0u8; 1024];

        let bytes_read: usize = socket.read(&mut input_buffer).await.unwrap();

        socket.write_all(&input_buffer[..bytes_read]).await.unwrap();
    }
}
