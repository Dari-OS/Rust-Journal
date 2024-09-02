use std::{env, io};

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() -> io::Result<()> {
    let port = env::args().nth(1).unwrap_or("8080".to_string());
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("The server is up and running on port {port}!");
    loop {
        let (connection, addr) = listener.accept().await?;
        println!("A client connected with the address: {addr}");

        tokio::spawn(async move {
            if let Err(e) = handle_connection(connection).await {
                eprintln!("An error occurred during a client connection: {}", e);
            }
        });
    }
}

async fn handle_connection(mut connection: TcpStream) -> io::Result<()> {
    let mut buf = vec![0; 1024];
    loop {
        let n = connection.read(&mut buf).await?;

        if n == 0 {
            return Ok(());
        }

        connection.write_all(&buf[0..n]).await?;
    }
}
