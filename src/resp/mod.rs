use aggregate::{BulkNullString, BulkString, RespFrameArray, RespNullArray};
use bytes::{Buf, BytesMut};
use enum_dispatch::enum_dispatch;
pub use frame::RespFrame;
use simple::{BigNumber, SimpleError, SimpleNull, SimpleString};
use thiserror::Error;

mod aggregate;
mod frame;
mod simple;

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();
const BUF_CAP: usize = 4096;

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    #[allow(dead_code)]
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    #[allow(dead_code)]
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[enum_dispatch]
pub trait RespEncode {
    fn encode(self) -> Vec<u8>;
}

#[derive(Error, Debug, PartialEq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    #[error("Invalid frame type: {0}")]
    InvalidFrameType(String),
    #[error("Invalid frame length： {0}")]
    InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
    #[error("Parse big number error: {0}")]
    ParseBigNumberError(#[from] bigdecimal::ParseBigDecimalError),
}

// utility functions
fn extract_fixed_data(
    buf: &mut BytesMut,
    expect: &str,
    expect_type: &str,
) -> Result<(), RespError> {
    if buf.len() < expect.len() {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(expect.as_bytes()) {
        return Err(RespError::InvalidFrame(format!(
            "Expect {} but got {:?}",
            expect_type, buf
        )));
    }

    // skip the prefix
    buf.advance(expect.len());
    Ok(())
}

pub fn extract_simple_data_end_index(buf: &[u8], prefix: &str) -> Result<usize, RespError> {
    if buf.len() < 3 {
        return Err(RespError::NotComplete);
    }
    if !buf.starts_with(prefix.as_bytes()) {
        return Err(RespError::InvalidFrame(format!("Invalid frame: {:?}", buf)));
    }
    let end_index = find_ctrl_index(buf, 1).ok_or(RespError::NotComplete)?;
    Ok(end_index)
}

fn find_ctrl_index(buf: &[u8], start: usize) -> Option<usize> {
    (start..buf.len()).find(|&i| buf[i] == b'\r' && buf.get(i + 1) == Some(&b'\n'))
}

pub fn parse_length(buf: &[u8], prefix: &str) -> Result<(usize, usize), RespError> {
    let end = extract_simple_data_end_index(buf, prefix)?;
    let s = String::from_utf8_lossy(&buf[prefix.len()..end]);
    Ok((end, s.parse()?))
}

fn calc_total_length(buf: &[u8], end: usize, len: usize, prefix: &str) -> Result<usize, RespError> {
    let mut total = end + CRLF_LEN;
    let mut data = &buf[total..];
    match prefix {
        "*" | "~" => {
            // find nth CRLF in the buffer, for array and set, we need to find 1 CRLF for each element
            for _ in 0..len {
                let len = RespFrame::expect_length(data)?;
                data = &data[len..];
                total += len;
            }
            Ok(total)
        }
        "%" => {
            // find nth CRLF in the buffer. For map, we need to find 2 CRLF for each key-value pair
            for _ in 0..len {
                let len = SimpleString::expect_length(data)?;

                data = &data[len..];
                total += len;

                let len = RespFrame::expect_length(data)?;
                data = &data[len..];
                total += len;
            }
            Ok(total)
        }
        _ => Ok(len + CRLF_LEN),
    }
}
