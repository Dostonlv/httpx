use std::{
    io::{Read, Write},
    net::TcpListener,
};

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let addr = listener.local_addr()?;

    println!("TCP server started: {addr}");

    let mut buf = [0u8; 1024];
    for stream in listener.incoming() {
        let mut stream = stream?;
        let n = stream.read(&mut buf)?;

        let read_input = &buf[..n];
        let send_text = String::from_utf8(read_input.to_vec())?;

        // GET / HTTP/1.1
        //  method path version

        let splits : Vec<&str> = send_text.split(' ').collect();
        println!("splits {splits:?}");

        let _ = stream.write(b"hello world")?;

        println!("Client connected: {:?}", stream.peer_addr()?);
        println!("Client send text: {send_text}");
    }

    Ok(())
}
