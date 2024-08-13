use crate::resp::{RespDecode, RespEncode};

use super::extract_simple_data_end_index;

impl RespDecode for f64 {
    const PREFIX: &'static str = ",";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, crate::resp::RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        let data = buf.split_to(end_idx + crate::resp::CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end_idx]);
        Ok(s.parse::<f64>()?)
    }

    fn expect_length(buf: &[u8]) -> Result<usize, crate::resp::RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        Ok(end_idx + crate::resp::CRLF_LEN)
    }
}

impl RespEncode for f64 {
    fn encode(self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(32);
        let ret = if self.abs() > 1e+8 || self.abs() < 1e-8 {
            format!(",{:+e}\r\n", self)
        } else {
            let sign = if self < 0.0 { "" } else { "+" };
            format!(",{}{}\r\n", sign, self)
        };

        buf.extend_from_slice(&ret.into_bytes());
        buf
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::frame::RespFrame;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_double_encode() {
        let frame: RespFrame = 123.456.into();
        assert_eq!(frame.encode(), b",+123.456\r\n");

        let frame: RespFrame = (-123.456).into();
        assert_eq!(frame.encode(), b",-123.456\r\n");

        let frame: RespFrame = 1.23456e+8.into();
        assert_eq!(frame.encode(), b",+1.23456e8\r\n");

        let frame: RespFrame = (-1.23456e-9).into();
        assert_eq!(&frame.encode(), b",-1.23456e-9\r\n");
    }

    #[test]
    fn test_double_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b",123.45\r\n");

        let frame = f64::decode(&mut buf)?;
        assert_eq!(frame, 123.45);

        buf.extend_from_slice(b",+1.23456e-9\r\n");
        let frame = f64::decode(&mut buf)?;
        assert_eq!(frame, 1.23456e-9);

        Ok(())
    }
}
