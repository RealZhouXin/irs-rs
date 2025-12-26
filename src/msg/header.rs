use crc::{CRC_16_ARC, Crc};
use tracing::{debug, info};
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

impl TryFrom<u8> for MsgType {
    type Error = ();

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MsgType::Undefined),
            1 => Ok(MsgType::Connect),
            2 => Ok(MsgType::ConnectAck),
            3 => Ok(MsgType::Data),
            14 => Ok(MsgType::DisConnect),
            16 => Ok(MsgType::ConnectExtended),
            17 => Ok(MsgType::ConnectExtendedAck),
            18 => Ok(MsgType::DisConnectExtended),
            _ => Err(()),
        }
    }
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum DeviceCode {
    MobileApp = 0x41,
    Backend = 0x42,
    ChargingStationApplicationSw = 0x43,
    MowerMainBoardApplicationSw = 0x4D,
    PcConnectedToMowerCsConnector = 0x4E,
    PcConnectedToMainBoardUartInterface = 0x4F,
    PcConnectedToCsBoard = 0x50,
}

#[derive(Debug)]
pub struct Header {
    soh: u8,
    stx: u8,
    pub msg_type: MsgType,
    pub payload_length: u16,
    pub crc: u16,
}
impl Header {
    pub fn new() -> Self {
        Header {
            soh: 0x01,
            stx: 0x02,
            msg_type: MsgType::Data,
            payload_length: 0,
            crc: 0,
        }
    }

    // 在message的to bytes中调用
    fn calculate_crc(&mut self, buf: &mut Vec<u8>) -> u16 {
        let crc = Crc::<u16>::new(&CRC_16_ARC);
        self.crc = crc.checksum(buf);
        buf[5..6].copy_from_slice(&self.crc.to_le_bytes());
        self.crc
    }
}
impl Into<Vec<u8>> for Header {
    fn into(self) -> Vec<u8> {
        let mut bytes = vec![0u8; 7];
        bytes[0] = self.soh;
        bytes[1] = self.stx;
        bytes[2] = self.msg_type as u8;
        bytes[3..5].copy_from_slice(&self.payload_length.to_le_bytes());
        bytes[5..7].copy_from_slice(&self.crc.to_le_bytes());
        bytes
    }
}
impl From<&[u8]> for Header {
    fn from(bytes: &[u8]) -> Self {
        let mut header = Header::new();
        header.soh = bytes[0];
        header.stx = bytes[1];
        header.msg_type = MsgType::try_from(bytes[2]).unwrap_or(MsgType::Undefined);
        header.payload_length = u16::from_le_bytes([bytes[3], bytes[4]]);
        header.crc = u16::from_le_bytes([bytes[5], bytes[6]]);
        header
    }
}
const DEFAULT_PROTOCOL_ID: u8 = 0x06; // production interface
const DEFAULT_PROTOCOL_VERSION: u8 = 0x02;
const DEFAULT_KEEP_ALIVE_LSB: u8 = 0;
const DEFAULT_KEEP_ALIVE_MSB: u8 = 0;
const DEFAULT_CLIENT_ID: u32 = 0x01;
const DEFAULT_SENDER: u8 = DeviceCode::PcConnectedToMainBoardUartInterface as u8;
const DEFAULT_RECEIVER: u8 = DeviceCode::MowerMainBoardApplicationSw as u8;
const DEFAULT_CONNECT_RETURN_CODE: u8 = 0x09;

#[derive(Debug)]
pub struct VarHeader {
    pub protocol_id: Option<u8>,
    pub protocol_version: Option<u8>,
    pub keepalive_lsb: Option<u8>,
    pub keepalive_msb: Option<u8>,
    pub sender: Option<u8>,
    pub receiver: Option<u8>,
    pub client_id: Option<u32>,
    pub connect_return_code: Option<u8>,
    pub size: u16,
    pub data: Vec<u8>,
}

impl VarHeader {
    pub fn new() -> Self {
        VarHeader {
            protocol_id: Some(DEFAULT_PROTOCOL_ID),
            protocol_version: Some(DEFAULT_PROTOCOL_VERSION),
            keepalive_lsb: Some(DEFAULT_KEEP_ALIVE_LSB),
            keepalive_msb: Some(DEFAULT_KEEP_ALIVE_MSB),
            sender: Some(DEFAULT_SENDER),
            receiver: Some(DEFAULT_RECEIVER),
            client_id: Some(DEFAULT_CLIENT_ID),
            connect_return_code: Some(DEFAULT_CONNECT_RETURN_CODE),
            size: 0, // TODO: has default size depending on message type
            data: Vec::new(),
        }
    }
    pub fn with_protocol_id(mut self, v: u8) -> Self {
        self.protocol_id = Some(v);
        self
    }
    pub fn with_protocol_version(mut self, v: u8) -> Self {
        self.protocol_version = Some(v);
        self
    }
    pub fn with_keepalive_lsb(mut self, v: u8) -> Self {
        self.keepalive_lsb = Some(v);
        self
    }
    pub fn with_keepalive_msb(mut self, v: u8) -> Self {
        self.keepalive_msb = Some(v);
        self
    }
    pub fn with_sender(mut self, v: u8) -> Self {
        self.sender = Some(v);
        self
    }
    pub fn with_receiver(mut self, v: u8) -> Self {
        self.receiver = Some(v);
        self
    }
    pub fn with_client_id(mut self, v: u32) -> Self {
        self.client_id = Some(v);
        self
    }
    pub fn with_connect_return_code(mut self, v: u8) -> Self {
        self.connect_return_code = Some(v);
        self
    }

    pub fn set_client_id(&mut self, v: u32) {
        self.client_id = Some(v);
    }

    pub fn build(mut self, msg_type: MsgType) -> Self {
        self.size = VarHeader::default_size(msg_type).unwrap();
        match msg_type {
            MsgType::ConnectExtended => {
                self.data.push(self.protocol_id.unwrap());
                self.data.push(DEFAULT_PROTOCOL_VERSION);
                self.data.push(DEFAULT_KEEP_ALIVE_LSB);
                self.data.push(DEFAULT_KEEP_ALIVE_MSB);
                self.data.extend(self.client_id.unwrap().to_le_bytes());
                self.data.push(self.sender.unwrap());
                self.data.push(self.receiver.unwrap());
            }
            MsgType::Connect => {
                self.data.push(self.protocol_id.unwrap());
                self.data.push(DEFAULT_PROTOCOL_VERSION);
                self.data.push(DEFAULT_KEEP_ALIVE_LSB);
                self.data.push(DEFAULT_KEEP_ALIVE_MSB);
                self.data.extend(self.client_id.unwrap().to_le_bytes());
                self.data.push(self.sender.unwrap());
            }
            MsgType::ConnectExtendedAck => {
                self.data.extend(self.client_id.unwrap().to_le_bytes());
                self.data.push(self.sender.unwrap());
                self.data.push(self.receiver.unwrap());
            }
            MsgType::Data => {
                self.data.extend(self.client_id.unwrap().to_le_bytes());
                self.data.push(self.sender.unwrap());
                self.data.push(self.receiver.unwrap());
            }
            MsgType::DisConnectExtended => {
                self.data.extend(self.client_id.unwrap().to_le_bytes());
                self.data.push(self.sender.unwrap());
                self.data.push(self.receiver.unwrap());
            }
            MsgType::DisConnect => {
                self.data.extend(self.client_id.unwrap().to_le_bytes());
                self.data.push(self.sender.unwrap());
                self.data.push(self.receiver.unwrap());
            }
            _ => {}
        };
        self
    }
    pub fn default_size(msg_type: MsgType) -> Option<u16> {
        match msg_type {
            MsgType::Connect => Some(9),
            MsgType::ConnectAck => Some(1),
            MsgType::Data => Some(6),
            MsgType::DisConnect => Some(5),
            MsgType::ConnectExtended => Some(10),
            MsgType::ConnectExtendedAck => Some(7),
            MsgType::DisConnectExtended => Some(6),
            _ => None,
        }
    }

    pub fn from_bytes(buf: &[u8], msg_type: MsgType) -> VarHeader {
        let size = Self::default_size(msg_type);
        let mut var_header = match msg_type {
            MsgType::ConnectExtended => Self::create_connect(buf),
            MsgType::Connect => Self::create_connect_legacy(buf),
            MsgType::ConnectExtendedAck => Self::create_connect_ack(buf),
            MsgType::ConnectAck => Self::create_connect_ack_lagacy(buf),
            MsgType::Data => Self::create_data(buf),
            MsgType::DisConnectExtended => Self::create_disconnect(buf),
            MsgType::DisConnect => Self::create_disconnect_legacy(buf),
            _ => VarHeader::new(),
        };
        var_header.size = size.unwrap();
        var_header
    }
    fn create_connect(buf: &[u8]) -> VarHeader {
        let var_header = VarHeader::new()
            .with_protocol_id(buf[0])
            .with_protocol_version(buf[1])
            .with_keepalive_lsb(buf[2])
            .with_keepalive_msb(buf[3])
            .with_client_id(u32::from_le_bytes(buf[4..8].try_into().unwrap()))
            .with_sender(buf[8])
            .with_receiver(buf[9]);
        var_header
    }
    fn create_connect_legacy(buf: &[u8]) -> VarHeader {
        VarHeader::new()
            .with_protocol_id(buf[0])
            .with_protocol_version(buf[1])
            .with_keepalive_lsb(buf[2])
            .with_keepalive_msb(buf[3])
            .with_client_id(u32::from_le_bytes(buf[4..8].try_into().unwrap()))
            .with_sender(buf[8])
    }
    fn create_connect_ack(buf: &[u8]) -> VarHeader {
        VarHeader::new()
            .with_connect_return_code(buf[0])
            .with_client_id(u32::from_le_bytes(buf[1..5].try_into().unwrap()))
            .with_sender(buf[5])
            .with_receiver(buf[6])
    }

    fn create_connect_ack_lagacy(buf: &[u8]) -> VarHeader {
        VarHeader::new().with_connect_return_code(buf[0])
    }

    fn create_data(buf: &[u8]) -> VarHeader {
        VarHeader::new()
            .with_client_id(u32::from_le_bytes(buf[0..4].try_into().unwrap()))
            .with_sender(buf[4])
            .with_receiver(buf[5])
    }
    fn create_disconnect(buf: &[u8]) -> VarHeader {
        VarHeader::new()
            .with_client_id(u32::from_le_bytes(buf[0..4].try_into().unwrap()))
            .with_sender(buf[4])
    }
    fn create_disconnect_legacy(buf: &[u8]) -> VarHeader {
        VarHeader::new()
            .with_client_id(u32::from_le_bytes(buf[0..4].try_into().unwrap()))
            .with_sender(buf[4])
            .with_receiver(buf[5])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_header() {
        crate::init_tracing();
        let mut header = Header::new();
        header.msg_type = MsgType::ConnectExtended;
        header.payload_length = 20;
        header.crc = 0x1234;
        let bytes: Vec<u8> = header.into();
        info!("header bytes: {bytes:?}");
        let h1 = Header::from(&bytes[..]);
        assert_eq!(h1.crc, 0x1234);
        assert_eq!(h1.payload_length, 20);
        assert_eq!(h1.msg_type, MsgType::ConnectExtended);
    }
    #[test]
    fn test_var_header_data() {
        crate::init_tracing();
        let mut var_header = VarHeader::new().build(MsgType::Data);
        assert_eq!(var_header.data.len(), 6);
        let vh = VarHeader::from_bytes(&var_header.data, MsgType::Data);
        assert_eq!(vh.client_id.unwrap(), DEFAULT_CLIENT_ID);
        assert_eq!(vh.sender.unwrap(), DEFAULT_SENDER);
        assert_eq!(vh.receiver.unwrap(), DEFAULT_RECEIVER);
        info!("vh: {vh:?}");
    }
}
