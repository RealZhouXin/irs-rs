mod msg;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use serde::{Serialize, Deserialize};

    use super::*;

    #[test]
    fn it_works() {
        let param = msg::params::data::Param471 {
            return_code: 0,
            default_cutting_height: 1,
            current_cutting_height: 1,
            information: 12,
        };
        
        let mut ser = msg::serialization::BinarySerializer::new();
        param.serialize(&mut ser).unwrap();
        let bytes = ser.into_bytes();
        println!("serialized: {bytes:?}");
        
        let mut de = msg::serialization::BinaryDeserializer::new(&bytes);
        let deserialized_param = msg::params::data::Param471::deserialize(&mut de).unwrap();
        println!("deserialized: {deserialized_param:?}");
        
        assert_eq!(param, deserialized_param);
    }
}
