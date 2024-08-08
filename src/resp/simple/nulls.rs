use crate::resp::{extract_fixed_data, RespDecode, RespEncode, RespError};

#[derive(Debug, PartialEq, PartialOrd)]
pub struct SimpleNull;

impl RespEncode for SimpleNull {
    fn encode(&self) -> Vec<u8> {
        b"_\r\n".to_vec()
    }
}

impl RespDecode for SimpleNull {
    const PREFIX: &'static str = "_";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        extract_fixed_data(buf, "_\r\n", "SimpleNull")?;
        Ok(SimpleNull)
    }
    fn expect_length(_buf: &[u8]) -> Result<usize, RespError> {
        Ok(3)
    }
}

#[cfg(test)]
mod tests {
    use crate::resp::frame::RespFrame;

    use super::*;
    use anyhow::Result;
    use bytes::BytesMut;

    #[test]
    fn test_simple_null_encode() {
        let frame: RespFrame = SimpleNull.into();

        assert_eq!(frame.encode(), b"_\r\n");
    }

    #[test]
    fn test_simple_null_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"_\r\n");

        let frame = SimpleNull::decode(&mut buf)?;
        assert_eq!(frame, SimpleNull);

        Ok(())
    }
}
