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
    id: u16,
    data: ParamPayload,
}

fn test_param() {
    let param = ParamPayload::P471(data::Param471 {
        return_code: 0,
        current_cutting_height: 0,
        default_cutting_height: 0,
        information: 1,
    });
    let bytes = param.encode();
    let p1 = ParamPayload::deconde(471, &bytes);
}
