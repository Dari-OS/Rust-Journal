mod tcp_connector;
use tcp_connector::TcpConnection;
fn main() {
    let connection = TcpConnection::new();
    let mut connection = connection.set_host("127.0.0.1:8080").unwrap();
    let _ = connection.send("Hello Rust!".as_bytes());
    let mut received = [0u8; 1024];

    let read = connection.receive(&mut received);

    println!(
        "{}",
        String::from_utf8(received[..read.unwrap()].to_vec()).unwrap()
    );

    connection.close();
}
