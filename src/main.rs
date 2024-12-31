use std::{
    io::{self, Read, Write},
    net::{Shutdown, TcpListener},
    ops::Deref,
};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:9092").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let message_size = MessageSize::from_reader(&mut stream)?;
                let header = ReqHeader::from_reader(&mut stream)?;
                stream.write_all(&message_size.0.to_be_bytes())?;
                stream.write_all(&header.correlation_id.to_be_bytes())?;
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

#[derive(Debug, PartialEq, Eq)]
struct MessageSize(i32);

impl From<[u8; 4]> for MessageSize {
    fn from(value: [u8; 4]) -> Self {
        MessageSize(i32::from_be_bytes(value))
    }
}

impl MessageSize {
    fn from_reader<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut buffer = [0; 4];
        r.read_exact(&mut buffer)?;
        Ok(Self::from(buffer))
    }
}

impl Deref for MessageSize {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
struct ReqHeader {
    api_key: i16,
    api_version: i16,
    correlation_id: i32,
}

impl ReqHeader {
    #[inline(always)]
    fn get_api_key(&self) -> i16 {
        self.api_key
    }
    #[inline(always)]
    fn get_api_version(&self) -> i16 {
        self.api_version
    }
    #[inline(always)]
    fn get_correlation_id(&self) -> i32 {
        self.correlation_id
    }
    fn from_reader<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut buffer = [0; 4];
        r.read_exact(&mut buffer[0..2])?;
        let api_key = i16::from_be_bytes([buffer[0], buffer[1]]);
        r.read_exact(&mut buffer[0..2])?;
        let api_version = i16::from_be_bytes([buffer[0], buffer[1]]);
        r.read_exact(&mut buffer)?;
        let correlation_id = i32::from_be_bytes(buffer);
        Ok(Self {
            api_key,
            api_version,
            correlation_id,
        })
    }
}
