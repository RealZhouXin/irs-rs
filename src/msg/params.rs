use std::{any::Any, fmt::Debug, process::id};

use serde::{Deserialize, Serialize, ser::Error};

use crate::msg::serialization::{self, BinaryDeserializer, BinarySerializeError, BinarySerializer};

pub mod data;

#[repr(u16)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ParamId {
    GetCuttingHeightReq = 470,
    GetCuttingHeightResp,
}

#[derive(Debug)]
pub enum ParamPayload {
    P470,
    P471(data::Param471),
}

impl ParamPayload {
    pub fn deconde(id: u16, bytes: &[u8]) -> Result<Self, BinarySerializeError> {
        let p = match id {
            471 => {
                let data = serialization::deserialize(&bytes)?;
                return Ok(ParamPayload::P471(data));
            }
            _ => {
                return Err(BinarySerializeError::custom("not support"));
            }
        };
    }
    pub fn encode(&self) -> Vec<u8> {
        match self {
            ParamPayload::P470 => Vec::new(),
            ParamPayload::P471(data) => serialization::serialize(&data).unwrap(),
        }
    }
}

pub struct Param {
    pub id: u16,
    pub data: ParamPayload,
}

impl Param {
    pub fn new(id: u16, data: ParamPayload) -> Self {
        Param { id, data }
    }
    pub fn to_bytes(self) -> Vec<u8> {
        let mut buf = Vec::new();
        buf.extend_from_slice(&self.id.to_le_bytes());
        let data_bytes = self.data.encode();
        let len = data_bytes.len() as u16;
        println!("data_bytes len: {len}");
        buf.extend_from_slice(&len.to_le_bytes());
        buf.extend_from_slice(&data_bytes);
        buf
    }
    pub fn from_bytes(bytes: &[u8]) -> Result<Self, BinarySerializeError> {
        let id = u16::from_le_bytes([bytes[0], bytes[1]]);
        let len = u16::from_le_bytes([bytes[2], bytes[3]]);
        println!("id: {id}, len: {len}, bytes len: {}", bytes.len());
        let data = ParamPayload::deconde(id, &bytes[4..(4 + len) as usize])?;
        Ok(Param { id, data })
    }
}

pub fn test_param() {
    let param = ParamPayload::P471(data::Param471 {
        return_code: 0,
        current_cutting_height: 0,
        default_cutting_height: 0,
        information: 1,
    });
    let bytes = param.encode();
    let p1 = ParamPayload::deconde(471, &bytes);
    let param = Param::new(
        471,
        ParamPayload::P471(data::Param471 {
            return_code: 0,
            current_cutting_height: 0,
            default_cutting_height: 0,
            information: 1,
        }),
    );
    let bytes = param.to_bytes();
    
    let p2 = Param::from_bytes(&bytes).unwrap();
}
