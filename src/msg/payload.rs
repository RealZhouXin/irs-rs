use super::params;
use tracing::debug;
///\brief Payload format.
///\details
/// Payload format, see document DGE-RLM-0069
///
///    |--------|---------|---------- Parameter #1---------|-----|- Parameter #N -|---------|
///    | MsgId  | UeLen   | ParamId | ParamLen | ParamData | ... |                | CRC     |
///    | 8 bits | 16 bits | 16 bits | 16 bits  | x bits    | ... |                | 16 bits |
///    |--------+---------+---------+----------+-----------+-----+----------------+---------|
///
pub struct Payload {
    pub msg_id: u8,
    pub unencrypted_length: u16,
    pub params: Vec<params::Param>,
    pub crc: u16,
}
impl Payload {
    pub fn new() -> Self {
        Payload {
            msg_id: 0,
            unencrypted_length: 0,
            params: Vec::new(),
            crc: 0,
        }
    }
    pub fn add_param(&mut self, param: params::Param) -> &mut Self {
        self.params.push(param);
        self
    }
    pub fn get_params(&self) -> &Vec<params::Param> {
        &self.params
    }
    pub fn calc_crc(&mut self, buf: &[u8]) {
        todo!()
    }

    pub fn to_bytes(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.push(self.msg_id);
        let mut total_param_length = 0usize;
        let param_bytes_list: Vec<Vec<u8>> = self.params.into_iter().map(|param| {
            let id = param.id;
            let param_bytes = param.to_bytes();
            debug!("id: {id}, len: {}", param_bytes.len());
            debug!("param bytes: {param_bytes:?}");
            total_param_length += param_bytes.len();
            param_bytes
        }).collect();
        buf.extend_from_slice(&(total_param_length as u16).to_le_bytes());
        param_bytes_list.into_iter().for_each(|param_bytes| {
            buf.extend_from_slice(&param_bytes);
        });
        // TODO: calculate CRC
        buf.extend_from_slice(&self.crc.to_le_bytes());
        buf
    }
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut payload = Payload::new();
        payload.msg_id = bytes[0];
        payload.unencrypted_length = u16::from_le_bytes([bytes[1], bytes[2]]);
        
        let mut index = 3;
        while index < bytes.len() - 2 {
            let id = u16::from_le_bytes([bytes[index], bytes[index + 1]]);
            let len = u16::from_le_bytes([bytes[index + 2], bytes[index + 3]]);
            let param_bytes = &bytes[index..(index + 4 + len as usize)];
            let param = params::Param::from_bytes(param_bytes).unwrap();
            payload.params.push(param);
            index += 4 + len as usize;
        }
        payload.crc = u16::from_le_bytes([bytes[bytes.len() - 2], bytes[bytes.len() - 1]]);
        payload
    }
}
