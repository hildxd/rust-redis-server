use enum_dispatch::enum_dispatch;

use super::simple::{SimpleError, SimpleNull, SimpleString};

#[derive(Debug)]
#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    SimpleStrings(SimpleString),
    SimpleErrors(SimpleError),
    SimpleIntegers(i64),
    SimpleNulls(SimpleNull),
    SimpleBooleans(bool),
}
