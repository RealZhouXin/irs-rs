pub mod header;
pub mod params;
pub mod payload;
pub mod serialization;

use header::MsgType;
use params::Param;

use crate::msg::{
    header::{Header, VarHeader},
    payload::Payload,
};

pub struct Msg {
    header: header::Header,
    var_header: header::VarHeader,
    payload: payload::Payload,
    body: Vec<u8>,
}

impl Msg {
    pub fn new() -> Self {
        Self {
            header: Header::new(),
            var_header: VarHeader::new(),
            payload: Payload::new(),
            body: Vec::new(),
        }
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
    pub fn set_client_id(&mut self, client_id: u32) {
        self.var_header.set_client_id(client_id);
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut buf = Vec::new();
        let msg_type = self.header.msg_type;
        let header_bytes: Vec<u8> = self.header.into();
        buf.extend_from_slice(&header_bytes);
        let var_header = self.var_header.build(msg_type);
        buf.extend_from_slice(&var_header.data);
        let payload = self.payload.to_bytes();
        buf.extend_from_slice(&payload);
        buf
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header = Header::from(&bytes[0..8]);
        let var_header = VarHeader::from_bytes(&bytes[8..], header.msg_type);
        let var_head_len = VarHeader::default_size(header.msg_type);
        let payload = Payload::from_bytes(&bytes[(7 + var_head_len.unwrap()) as usize..]);
        Self {
            header,
            var_header,
            payload,
            body: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
   #[test] 
   fn test_msg_ser() {
       
   }
}