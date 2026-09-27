use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("TCP server started: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let mut stream = stream?;
        handle_stream(&mut stream);
        stream.write_all(b"hello world")?;
    }

    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> anyhow::Result<(), io::Error> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];

    loop {
        let n = stream.read(&mut chunk);

        if n == 0 {
            break;
        }

        buf.extend_from_slice(&chunk[..n]);

        if buf.windows(4).any(|window| window == b"\r\n\r\n") {
            break;
        }
    }

    let send_text = String::from_utf8(buf)?;

    println!("Client send text:\n{send_text}");

    // GET /hello HTTP/1.1
    // method path version

    let mut lines = send_text.split("\r\n");

    let request_line = lines.next().unwrap();

    let mut parts = request_line.split_whitespace();

    let method = parts.next().unwrap();
    let path = parts.next().unwrap();
    let version = parts.next().unwrap();

    println!("method: {method}");
    println!("path: {path}");
    println!("version: {version}");

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let request_line = lines.next().unwrap();
        if request_line == "" {
            break;
        }

        let parts = request_line.split_once(":");

        let key = parts.unwrap().0;
        let value = parts.unwrap().1;
        headers.insert(key.to_string(), value.to_string());
    }
    println!("{headers:?}");

    Ok(())
}
