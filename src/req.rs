use std::{io, ops::Deref};

#[derive(Debug, PartialEq, Eq)]
pub struct MessageSize(i32);

impl From<[u8; 4]> for MessageSize {
    fn from(value: [u8; 4]) -> Self {
        MessageSize(i32::from_be_bytes(value))
    }
}

impl MessageSize {
    pub fn from_reader<R: io::Read>(r: &mut R) -> io::Result<Self> {
        let mut buffer = [0; 4];
        r.read_exact(&mut buffer)?;
        Ok(Self::from(buffer))
    }
    #[inline(always)]
    pub fn as_i32(&self) -> i32 {
        self.0
    }
}

impl Deref for MessageSize {
    type Target = i32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct ReqHeader {
    api_key: i16,
    api_version: i16,
    correlation_id: i32,
}

impl ReqHeader {
    #[inline(always)]
    pub fn get_api_key(&self) -> i16 {
        self.api_key
    }
    #[inline(always)]
    pub fn get_api_version(&self) -> i16 {
        self.api_version
    }
    #[inline(always)]
    pub fn get_correlation_id(&self) -> i32 {
        self.correlation_id
    }
    pub fn from_reader<R: io::Read>(r: &mut R) -> io::Result<Self> {
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
