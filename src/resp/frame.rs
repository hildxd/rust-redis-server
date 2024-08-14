use enum_dispatch::enum_dispatch;

use super::{
    aggregate::{BulkNullString, BulkString, RespFrameArray, RespNullArray},
    simple::{BigNumber, SimpleError, SimpleNull, SimpleString},
    RespDecode, RespError,
};

#[enum_dispatch(RespEncode)]
#[derive(Debug, PartialEq, PartialOrd)]
pub enum RespFrame {
    Strings(SimpleString),
    Errors(SimpleError),
    Integers(i64),
    Nulls(SimpleNull),
    Booleans(bool),
    Doubles(f64),
    BigNumbers(BigNumber),
    BulkStrings(BulkString),
    BulkNullStrings(BulkNullString),
    NullArray(RespNullArray),
    Array(RespFrameArray),
}

impl RespDecode for RespFrame {
    const PREFIX: &'static str = "";
    fn decode(buf: &mut bytes::BytesMut) -> Result<Self, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => SimpleString::decode(buf).map(RespFrame::Strings),
            Some(b'-') => SimpleError::decode(buf).map(RespFrame::Errors),
            Some(b':') => i64::decode(buf).map(RespFrame::Integers),
            Some(b'$') => match BulkNullString::decode(buf) {
                Ok(frame) => Ok(RespFrame::BulkNullStrings(frame)),
                Err(RespError::NotComplete) => Err(RespError::NotComplete),
                Err(_) => {
                    let frame = BulkString::decode(buf)?;
                    Ok(frame.into())
                }
            },
            Some(b'*') => match RespNullArray::decode(buf) {
                Ok(frame) => Ok(RespFrame::NullArray(frame)),
                Err(RespError::NotComplete) => Err(RespError::NotComplete),
                Err(_) => {
                    let frame = RespFrameArray::decode(buf)?;
                    Ok(frame.into())
                }
            },
            Some(b'_') => SimpleNull::decode(buf).map(RespFrame::Nulls),
            Some(b'#') => bool::decode(buf).map(RespFrame::Booleans),
            Some(b',') => f64::decode(buf).map(RespFrame::Doubles),
            None => Err(RespError::NotComplete),
            _ => Err(RespError::InvalidFrameType(format!(
                "expect_length: unknown frame type: {:?}",
                buf
            ))),
        }
    }
    fn expect_length(buf: &[u8]) -> Result<usize, RespError> {
        let mut iter = buf.iter().peekable();
        match iter.peek() {
            Some(b'+') => SimpleString::expect_length(buf),
            Some(b'-') => SimpleError::expect_length(buf),
            Some(b':') => i64::expect_length(buf),
            Some(b'$') => BulkString::expect_length(buf),
            Some(b'*') => RespFrameArray::expect_length(buf),
            Some(b'_') => SimpleNull::expect_length(buf),
            Some(b'#') => bool::expect_length(buf),
            Some(b',') => f64::expect_length(buf),
            _ => Err(RespError::NotComplete),
        }
    }
}

impl<const N: usize> From<&[u8; N]> for RespFrame {
    fn from(buf: &[u8; N]) -> Self {
        BulkString(buf.to_vec()).into()
    }
}
