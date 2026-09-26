use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let addr = listener.local_addr()?;

    println!("TCP server started: {addr}");

    for stream in listener.incoming() {
        let stream = stream?;

        println!("Client connected: {:?}", stream.peer_addr()?);
    }

    Ok(())
}
