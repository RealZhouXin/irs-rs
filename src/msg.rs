mod header;
mod params;
mod payload;

use header::MsgType;
use params::Param;

struct Msg {
    header: header::Header,
    var_header: header::VarHeader,
    payload: payload::Payload,
    body: Vec<u8>,
}

impl Msg {
    fn new() -> Self {
        todo!()
    }
    fn set_message_type(&mut self, msg_type: MsgType) {
        self.header.msg_type = msg_type;
    }
    fn get_message_type(&self) -> MsgType {
        self.header.msg_type
    }
    fn add_param(&mut self, param: Param) {
        todo!()
    }
    fn get_param(&self, index: usize) -> &Param {
        todo!()
    }
}
impl Into<Vec<u8>> for Msg {
    fn into(self) -> Vec<u8> {
        todo!()
    }
}
impl From<Vec<u8>> for Msg {
    fn from(bytes: Vec<u8>) -> Self {
        todo!()
    }
}
