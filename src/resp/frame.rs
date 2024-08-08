use enum_dispatch::enum_dispatch;

use super::simple::{SimpleError, SimpleString};

#[derive(Debug)]
#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    SimpleStrings(SimpleString),
    SimpleErrors(SimpleError),
}
