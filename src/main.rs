use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
};

use anyhow::anyhow;

struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}
fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("TCP server started: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        let mut stream = stream?;
        let resp = handle_stream(&mut stream);
        stream.write_all(b"hello world")?;
    }

    Ok(())
}

fn handle_stream(stream: &mut TcpStream) -> anyhow::Result<Request> {
    let mut buf = Vec::new();
    let mut chunk = [0u8; 1024];

    loop {
        let n = stream.read(&mut chunk)?;

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

    let request_line = lines
        .next()
        .ok_or_else(|| anyhow!("error while read request lines"))?;
    let mut parts = request_line.split_whitespace();

    let method = parts
        .next()
        .ok_or_else(|| anyhow!("error while parsing method "))?;
    let path = parts
        .next()
        .ok_or_else(|| anyhow!("error while parsing path "))?;
    let version = parts
        .next()
        .ok_or_else(|| anyhow!("error while parsing version "))?;

    let mut headers: HashMap<String, String> = HashMap::new();

    loop {
        let request_line = lines
            .next()
            .ok_or_else(|| anyhow!("error while reading request line "))?;
        if request_line == "" {
            break;
        }

        let parts = request_line
            .split_once(":")
            .ok_or_else(|| anyhow!("error while parsing request line's parts "))?;

        let key = parts.0;
        let value = parts.1;
        headers.insert(key.trim().to_string(), value.trim().to_string());
    }

    Ok(Request {
        method: method.to_string(),
        path: path.to_string(),
        version: version.to_string(),
        headers,
    })
}
