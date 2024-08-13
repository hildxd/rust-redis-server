use anyhow::Result;
use std::str::FromStr;

use bigdecimal::BigDecimal;

use crate::resp::{RespDecode, RespEncode, RespError, CRLF_LEN};

use super::extract_simple_data_end_index;

#[derive(Debug)]
pub struct BigNumber(pub BigDecimal);

impl BigNumber {
    #[allow(unused)]
    pub fn new(s: &str) -> Result<BigNumber, RespError> {
        let num = BigDecimal::from_str(s)?;
        Ok(BigNumber(num))
    }
}

impl RespEncode for BigNumber {
    fn encode(self) -> Vec<u8> {
        let sign = match self.0 >= BigDecimal::from(0) {
            true => "+",
            false => "",
        };
        format!("({}{}\r\n", sign, self.0).into_bytes()
    }
}

impl RespDecode for BigNumber {
    const PREFIX: &'static str = "(";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        let data = buf.split_to(end_idx + CRLF_LEN);
        let s = String::from_utf8_lossy(&data[Self::PREFIX.len()..end_idx]);
        let num = BigDecimal::from_str(&s)?;
        Ok(BigNumber(num))
    }

    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let end_idx = extract_simple_data_end_index(buf, Self::PREFIX)?;
        Ok(end_idx + CRLF_LEN)
    }
}

#[cfg(test)]
mod test {
    use crate::resp::frame::RespFrame;

    use super::*;
    use anyhow::Result;

    #[test]
    fn test_big_numbers_encode() -> Result<()> {
        let frame: RespFrame = BigNumber::new("222")?.into();
        assert_eq!(frame.encode(), b"(+222\r\n");

        let frame: RespFrame = BigNumber::new("-2222.122")?.into();
        assert_eq!(frame.encode(), b"(-2222.122\r\n");

        assert!(BigNumber::new("s21s").is_err());
        Ok(())
    }

    #[test]
    fn test_big_numbers_decode() -> Result<()> {
        let mut buf = bytes::BytesMut::new();
        buf.extend_from_slice(b"(+222\r\n");

        let frame = BigNumber::decode(&mut buf)?;
        assert_eq!(frame.0, BigDecimal::from(222));

        let mut buf = bytes::BytesMut::new();
        buf.extend_from_slice(b"(-2222.122\r\n");

        let frame = BigNumber::decode(&mut buf)?;
        assert_eq!(frame.0, BigDecimal::from_str("-2222.122")?);

        Ok(())
    }
}
