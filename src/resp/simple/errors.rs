use std::ops::Deref;

use crate::resp::{RespDecode, RespEncode, RespError, CRLF_LEN};

use super::extract_simple_data_end_index;

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SimpleError(pub String);

impl SimpleError {
    pub fn new(s: impl Into<String>) -> Self {
        SimpleError(s.into())
    }
}
impl From<&str> for SimpleError {
    fn from(s: &str) -> Self {
        SimpleError(s.to_string())
    }
}

impl Deref for SimpleError {
    type Target = String;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl RespEncode for SimpleError {
    fn encode(&self) -> Vec<u8> {
        format!("-{}\r\n", self.0).into_bytes()
    }
}

impl RespDecode for SimpleError {
    const PREFIX: &'static str = "-";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        let data = buf.split_to(end_idx + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end_idx]);
        Ok(SimpleError::new(s.to_string()))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        Ok(end_idx + CRLF_LEN)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::frame::RespFrame;

    use super::*;
    use anyhow::Result;
    use bytes::{BufMut, BytesMut};

    #[test]
    fn test_simple_error_encode() {
        let frame: RespFrame = SimpleError::new("ERROR".to_string()).into();

        assert_eq!(frame.encode(), b"-ERROR\r\n");
    }

    #[test]
    fn test_simple_error_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"-ERROR\r\n");

        let frame = SimpleError::decode(&mut buf)?;
        assert_eq!(frame, SimpleError::new("ERROR".to_string()));

        buf.extend_from_slice(b"-hello\r");

        let ret = SimpleError::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let frame = SimpleError::decode(&mut buf)?;
        assert_eq!(frame, SimpleError::new("hello".to_string()));

        Ok(())
    }
}
