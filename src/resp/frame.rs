use enum_dispatch::enum_dispatch;

use super::simple::{SimpleError, SimpleNull, SimpleString};

#[derive(Debug)]
#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    Strings(SimpleString),
    Errors(SimpleError),
    Integers(i64),
    Nulls(SimpleNull),
    Booleans(bool),
    Doubles(f64),
}
