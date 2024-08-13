mod big_numbers;
mod booleans;
mod doubles;
mod errors;
mod integer;
mod nulls;
mod strings;

pub use big_numbers::BigNumber;
pub use errors::SimpleError;
pub use nulls::SimpleNull;
pub use strings::SimpleString;

use super::RespError;

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
