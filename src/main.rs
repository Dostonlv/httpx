use std::{
    collections::HashMap,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use anyhow::anyhow;

#[derive(Debug)]
struct Request {
    pub method: String,
    pub path: String,
    pub version: String,
    pub headers: HashMap<String, String>,
}

// http 1.1 standarts
fn main() -> anyhow::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    println!("TCP server started: {}", listener.local_addr()?);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    if let Err(e) = handle_connection(stream) {
                        println!("[error] error while handle connection {e}");
                    }
                });
            }
            Err(err) => println!("error while accept stream {err}"),
        }
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
        headers.insert(key.trim().to_ascii_lowercase(), value.trim().to_string());
    }

    if !headers.contains_key("host") && version == "HTTP/1.1" {
        return Err(anyhow!("host header is must have"));
    }
    Ok(Request {
        method: method.to_string(),
        path: path.to_string(),
        version: version.to_string(),
        headers,
    })
}

fn write_response(
    stream: &mut TcpStream,
    status_code: u16,
    reason: &str,
    body: &str,
) -> io::Result<()> {
    let response = format!(
        "HTTP/1.1 {status_code} {reason}\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{body}",
        body.len()
    );

    stream.write_all(response.as_bytes())
}

fn handle_connection(mut stream: TcpStream) -> anyhow::Result<()> {
    stream.set_read_timeout(Some(Duration::from_secs(5)))?;
    match handle_stream(&mut stream) {
        Ok(req) => {
            println!("{req:?}");
            let (code, reason, body) = match (req.path.as_str(), req.method.as_str()) {
                ("/", "GET") => (200, "OK", "ok"),
                ("/hello", "GET") => (200, "OK", "hello world"),
                (_, "GET") => (404, "Not Found", "not found"),
                (_, _) => (405, "Method Not Allowed", "method not allowed"),
            };
            write_response(&mut stream, code, reason, body)?;
        }
        Err(e) => {
            println!("error: {e}");
            write_response(&mut stream, 400, "Bad Request", "bad request")?;
        }
    }

    Ok(())
}
