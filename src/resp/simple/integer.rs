use crate::resp::{RespDecode, RespEncode, RespError, CRLF_LEN};

use super::extract_simple_data_end_index;
impl RespEncode for i64 {
    fn encode(self) -> Vec<u8> {
        let sign = match self.is_negative() {
            true => "",
            false => "+",
        };
        format!(":{}{}\r\n", sign, self).into_bytes()
    }
}

impl RespDecode for i64 {
    const PREFIX: &'static str = ":";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        let data = buf.split_to(end_idx + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end_idx]);
        let i = s.parse()?;
        Ok(i)
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
    use bytes::BytesMut;

    #[test]
    fn test_integer_encode() {
        let frame: RespFrame = 100.into();

        assert_eq!(frame.encode(), b":+100\r\n");

        let frame: RespFrame = (-100).into();
        assert_eq!(frame.encode(), b":-100\r\n");
    }

    #[test]
    fn test_integer_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b":100\r\n");

        let frame = i64::decode(&mut buf)?;
        assert_eq!(frame, 100);

        Ok(())
    }
}
