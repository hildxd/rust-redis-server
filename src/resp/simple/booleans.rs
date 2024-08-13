use crate::resp::{extract_fixed_data, RespDecode, RespEncode, RespError, CRLF_LEN};

use super::extract_simple_data_end_index;
impl RespEncode for bool {
    fn encode(self) -> Vec<u8> {
        let value = match self {
            true => "t",
            false => "f",
        };
        format!("#{}\r\n", value).into_bytes()
    }
}

impl RespDecode for bool {
    const PREFIX: &'static str = "#";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        match extract_fixed_data(buf, "#t\r\n", "SimpleBooleans") {
            Ok(_) => Ok(true),
            Err(RespError::NotComplete) => Err(RespError::NotComplete),
            Err(_) => match extract_fixed_data(buf, "#f\r\n", "SimpleBooleans") {
                Ok(_) => Ok(false),
                Err(e) => Err(e),
            },
        }
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
    fn test_boolean_encode() {
        let frame: RespFrame = true.into();
        assert_eq!(frame.encode(), b"#t\r\n");

        let frame: RespFrame = false.into();
        assert_eq!(frame.encode(), b"#f\r\n");
    }

    #[test]
    fn test_boolean_decode() -> Result<()> {
        let mut buf = BytesMut::new();
        buf.extend_from_slice(b"#t\r\n");

        let frame = bool::decode(&mut buf)?;
        assert!(frame);

        buf.extend_from_slice(b"#f\r\n");

        let frame = bool::decode(&mut buf)?;
        assert!(!frame);

        buf.extend_from_slice(b"#f\r");
        let ret = bool::decode(&mut buf);
        assert_eq!(ret.unwrap_err(), RespError::NotComplete);

        buf.put_u8(b'\n');
        let frame = bool::decode(&mut buf)?;
        assert!(!frame);

        Ok(())
    }
}
