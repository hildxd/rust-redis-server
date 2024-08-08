use enum_dispatch::enum_dispatch;

use super::simple::SimpleString;

#[derive(Debug)]
#[enum_dispatch(RespEncode)]
pub enum RespFrame {
    SimpleStrings(SimpleString),
}
