pub mod header;
pub mod params;
pub mod payload;
pub mod serialization;

use header::MsgType;
use params::Param;

pub struct Msg {
    header: header::Header,
    var_header: header::VarHeader,
    payload: payload::Payload,
    body: Vec<u8>,
}

impl Msg {
    pub fn new() -> Self {
        todo!()
    }
    pub fn set_message_type(&mut self, msg_type: MsgType) {
        self.header.msg_type = msg_type;
    }
    pub fn get_message_type(&self) -> MsgType {
        self.header.msg_type
    }
    pub fn add_param(&mut self, param: Param) {
        self.payload.add_param(param);
    }
    pub fn get_param(&self, index: usize) -> &Param {
        self.payload.get_params().get(index).unwrap()
    }
    pub fn set_msg_id(&mut self, id: u8) {
        self.payload.msg_id = id;
    }
    pub fn get_msg_id(&self) -> u8 {
        self.payload.msg_id
    }
    pub fn set_client_id(&mut self, client_id: u32) {}
    pub fn to_bytes(&self) -> Vec<u8> {
        todo!()
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        todo!()
    }
}
