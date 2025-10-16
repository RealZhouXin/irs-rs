mod header;
mod payload;
mod params;
struct Msg {
    header: header::Header,
    var_header: header::VarHeader,
    payload: payload::Payload,
    body: Vec<u8>,
}

impl Msg {
    fn set_message_type(&mut self, msg_type: header::MsgType) {
        self.header.msg_type = msg_type;
    }
}