use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Param471 {
    pub return_code: u8,
    pub default_cutting_height: u8,
    pub current_cutting_height: u8,
    pub information: u8,
}
