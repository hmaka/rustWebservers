use std::net::SocketAddr;
use tokio::io::AsyncBufReadExt;

use tokio::{
    io::{AsyncWriteExt, BufReader},
    net::TcpListener,
    sync::broadcast,
};

#[tokio::main]
async fn main() -> ! {
    let listener: TcpListener = TcpListener::bind("localhost:7878").await.unwrap();
    let (tx, _rx) = broadcast::channel::<(String, SocketAddr)>(10);

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
                        if result.unwrap() == 0{
                            break;
                        }
                        tx.send((line.clone(), address)).unwrap();
                        line.clear();
                    }
                    result = rx.recv() =>{
                        let (msg, other_address) = result.unwrap();

                        if address != other_address{
                            writer.write_all(msg.as_bytes()).await.unwrap();
                        }
                    }
                }
            }
        });
    }
}
