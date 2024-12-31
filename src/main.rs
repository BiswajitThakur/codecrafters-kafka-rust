use std::{
    io::{self, Write},
    net::TcpListener,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                stream.write_all(b"\x00\x00\x00\x00")?;
                stream.write_all(b"\x00\x00\x00\x07")?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
