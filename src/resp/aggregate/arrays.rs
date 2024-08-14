use std::{fmt::Debug, ops::Deref};

use bytes::Buf;

use crate::{
    resp::{
        calc_total_length, extract_fixed_data, parse_length, RespDecode, RespEncode, BUF_CAP,
        CRLF_LEN,
    },
    RespFrame,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespFrameArray(pub Vec<RespFrame>);

#[derive(Debug, PartialEq, PartialOrd)]
pub struct RespNullArray;

impl RespDecode for RespFrameArray {
    const PREFIX: &'static str = "*";

    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::resp::RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        let total_len = calc_total_length(buf, end, len, Self::PREFIX)?;

        if buf.len() < total_len {
            return Err(crate::resp::RespError::NotComplete);
        }

        buf.advance(end + CRLF_LEN);
        let mut frames = Vec::with_capacity(len);
        for _ in 0..len {
            let frame = RespFrame::decode(buf)?;
            frames.push(frame);
        }

        Ok(RespFrameArray::new(frames))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, crate::resp::RespError> {
        let (end, len) = parse_length(buf, Self::PREFIX)?;
        calc_total_length(buf, end, len, Self::PREFIX)
    }
}

impl RespEncode for RespFrameArray {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(BUF_CAP);
        buf.extend_from_slice(&format!("*{}\r\n", self.0.len()).into_bytes());
        for frame in self.0 {
            buf.extend_from_slice(&frame.encode())
        }
        buf
    }
}

impl RespEncode for RespNullArray {
    fn encode(self) -> Vec<u8> {
        b"*-1\r\n".to_vec()
    }
}

impl RespDecode for RespNullArray {
    const PREFIX: &'static str = "*";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::resp::RespError> {
        extract_fixed_data(buf, "*-1\r\n", "NullArray")?;
        Ok(RespNullArray)
    }

    fn expect_length(_buf: &[u8]) -> Result<usize, crate::resp::RespError> {
        Ok(4)
    }
}

impl RespFrameArray {
    pub fn new(s: impl Into<Vec<RespFrame>>) -> Self {
        RespFrameArray(s.into())
    }
}

impl Deref for RespFrameArray {
    type Target = Vec<RespFrame>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<Vec<RespFrame>> for RespFrameArray {
    fn from(value: Vec<RespFrame>) -> Self {
        RespFrameArray::new(value)
    }
}

#[cfg(test)]
mod tests {

    use crate::resp::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_array_encode() {
        let frame: RespFrame = RespFrameArray::new(vec![
            BulkString::new("set".to_string()).into(),
            BulkString::new("hello".to_string()).into(),
            BulkString::new("world".to_string()).into(),
        ])
        .into();
        assert_eq!(
            &frame.encode(),
            b"*3\r\n$3\r\nset\r\n$5\r\nhello\r\n$5\r\nworld\r\n"
        );
    }

    #[test]
    fn test_null_array_encode() {
        let frame: RespFrame = RespNullArray.into();
        assert_eq!(frame.encode(), b"*-1\r\n");
    }

    #[test]
    fn test_null_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*-1\r\n");

        let frame = RespNullArray::decode(&mut buf)?;
        assert_eq!(frame, RespNullArray);

        Ok(())
    }

    #[test]
    fn test_array_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n$5\r\nhello\r\n");

        let frame = RespFrameArray::decode(&mut buf)?;
        println!("{:?}", frame);
        println!(
            "{:?}",
            RespFrameArray::new([b"set".into(), b"hello".into()])
        );
        assert_eq!(frame, RespFrameArray::new([b"set".into(), b"hello".into()]));

        buf.extend_from_slice(b"*2\r\n$3\r\nset\r\n");
        let ret = RespFrameArray::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.extend_from_slice(b"$5\r\nhello\r\n");
        let frame = RespFrameArray::decode(&mut buf)?;
        assert_eq!(frame, RespFrameArray::new([b"set".into(), b"hello".into()]));

        Ok(())
    }
}
