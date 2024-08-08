use bytes::BytesMut;
use enum_dispatch::enum_dispatch;
use frame::RespFrame;
use simple::{SimpleError, SimpleString};
use thiserror::Error;

mod aggregate;
mod frame;
mod simple;

const CRLF: &[u8] = b"\r\n";
const CRLF_LEN: usize = CRLF.len();

pub trait RespDecode: Sized {
    const PREFIX: &'static str;
    #[allow(dead_code)]
    fn decode(buf: &mut BytesMut) -> Result<Self, RespError>;
    #[allow(dead_code)]
    fn expect_length(buf: &[u8]) -> Result<usize, RespError>;
}

#[enum_dispatch]
pub trait RespEncode {
    fn encode(&self) -> Vec<u8>;
}

#[derive(Error, Debug, PartialEq, Eq)]
pub enum RespError {
    #[error("Invalid frame: {0}")]
    InvalidFrame(String),
    // #[error("Invalid frame type: {0}")]
    // InvalidFrameType(String),
    // #[error("Invalid frame length： {0}")]
    // InvalidFrameLength(isize),
    #[error("Frame is not complete")]
    NotComplete,

    #[error("Parse error: {0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("Utf8 error: {0}")]
    Utf8Error(#[from] std::string::FromUtf8Error),
    #[error("Parse float error: {0}")]
    ParseFloatError(#[from] std::num::ParseFloatError),
}
