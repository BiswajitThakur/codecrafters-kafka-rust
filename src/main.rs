use std::{
    io::{self, Write},
    net::{Shutdown, TcpListener},
};

use codecrafters_kafka::{ErrorCodes, MessageSize, ReqHeader};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message_size = MessageSize::from_reader(&mut stream)?;
                let header = ReqHeader::from_reader(&mut stream)?;
                stream.write_all(&message_size.as_i32().to_be_bytes())?;
                stream.write_all(&header.get_correlation_id().to_be_bytes())?;
                match header.get_api_version() {
                    0..4 => stream.write_all(&(ErrorCodes::None as i16).to_be_bytes())?,
                    _ => {
                        stream.write_all(&(ErrorCodes::UnsupportedVersion as i16).to_be_bytes())?
                    }
                }
                stream.flush().unwrap();
                stream.shutdown(Shutdown::Write).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
