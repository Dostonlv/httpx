use std::net::TcpListener;

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let b  = listener.local_addr()?;
    println!("tcp server started {b}");

    Ok(())
}
