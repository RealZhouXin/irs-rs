use super::params;

pub struct Payload {
    msg_id: u8,
    unencrypted_length: u16,
    params: Vec<params::Param>,
}