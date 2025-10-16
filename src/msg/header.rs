#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum MsgType {
    Undefined = 0,
    Connect = 1,
    ConnectAck = 2,
    Data = 3,
    DisConnect = 14,
    ConnectExtended = 16,
    ConnectExtendedAck = 17,
    DisConnectExtended = 18,
}

pub struct Header {
    soh: u8,
    stx: u8,
    pub msg_type: MsgType,
    pub payload_length: u16,
    pub crc: u16,
}
impl Header {
    fn new() -> Self {
        Header {
            soh: 0x01,
            stx: 0x02,
            msg_type: MsgType::Data,
            payload_length: 0,
            crc: 0,
        }
    }
}
impl Into<[u8; 7]> for Header {
    fn into(self) -> [u8; 7] {
        let mut bytes = [0u8; 7];
        bytes[0] = self.soh;
        bytes[1] = self.stx;
        bytes[2] = self.msg_type as u8;
        bytes[3..5].copy_from_slice(&self.payload_length.to_le_bytes());
        bytes[5..7].copy_from_slice(&self.crc.to_le_bytes());
        bytes
    }
}
const DEFAULT_PROTOCOL_ID: u8 = 0x06;
const DEFAULT_PROTOCOL_VERSION: u8 = 0x02;
const DEFAULT_KEEP_ALIVE_LSB: u8 = 0;
const DEFAULT_KEEP_ALIVE_MSB: u8 = 0;
const DEFAULT_CLIENT_ID: u32 = 0x01;
const DEFAULT_SENDER: u8 = 0x4f; // pc connect to mainboard UART interface
const DEFAULT_RECEIVER: u8 = 0x4d; // mainboard UART 
const DEFAULT_CONNECT_RETURN_CODE: u8 = 0x09; 

pub struct VarHeader {
    protocol_id: Option<u8>,
    protocol_version: Option<u8>,
    sender : Option<u8>,
    receiver : Option<u8>,
    client_id: Option<u32>,
    connect_return_code: Option<u8>,
    size: u16
}

impl VarHeader {
    fn new() -> Self {
        VarHeader {
            protocol_id: Some(DEFAULT_PROTOCOL_ID),
            protocol_version: Some(DEFAULT_PROTOCOL_VERSION),
            sender: Some(DEFAULT_SENDER),
            receiver: Some(DEFAULT_RECEIVER),
            client_id: Some(DEFAULT_CLIENT_ID),
            connect_return_code: Some(DEFAULT_CONNECT_RETURN_CODE),
            size: 0// TODO: has default size depending on message type
        }
    }
}
impl From<Vec<u8>> for VarHeader {
    fn from(bytes: Vec<u8>) -> Self {
        let mut var_header = VarHeader::new();
        let mut index = 0;

        if bytes.len() > index {
            var_header.protocol_id = Some(bytes[index]);
            index += 1;
        }
        if bytes.len() > index {
            var_header.protocol_version = Some(bytes[index]);
            index += 1;
        }
        if bytes.len() > index {
            var_header.sender = Some(bytes[index]);
            index += 1;
        }
        if bytes.len() > index {
            var_header.receiver = Some(bytes[index]);
            index += 1;
        }
        if bytes.len() >= index + 4 {
            var_header.client_id = Some(u32::from_le_bytes([
                bytes[index],
                bytes[index + 1],
                bytes[index + 2],
                bytes[index + 3],
            ]));
            index += 4;
        }
        if bytes.len() > index {
            var_header.connect_return_code = Some(bytes[index]);
            index += 1;
        }

        var_header.size = index as u16;
        var_header
    }
}
impl Into <Vec<u8>> for VarHeader {
    fn into(self) -> Vec<u8> {
        let mut bytes = Vec::new();

        if let Some(protocol_id) = self.protocol_id {
            bytes.push(protocol_id);
        }
        if let Some(protocol_version) = self.protocol_version {
            bytes.push(protocol_version);
        }
        if let Some(sender) = self.sender {
            bytes.push(sender);
        }
        if let Some(receiver) = self.receiver {
            bytes.push(receiver);
        }
        if let Some(client_id) = self.client_id {
            bytes.extend_from_slice(&client_id.to_le_bytes());
        }
        if let Some(connect_return_code) = self.connect_return_code {
            bytes.push(connect_return_code);
        }

        bytes
    }
}