mod msg;
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
fn init_tracing() {
    let _ = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .with_test_writer()
        .try_init();
}
#[cfg(test)]
mod tests {
    use tracing::info;

    use crate::msg::params::data;

    use super::*;

    #[test]
    fn it_works() {
        let param = msg::params::data::Param471 {
            return_code: 0,
            default_cutting_height: 1,
            current_cutting_height: 1,
            information: 12,
        };

        // let mut ser = msg::serialization::BinarySerializer::new();
        // param.serialize(&mut ser).unwrap();
        let bytes = msg::serialization::serialize(&param).unwrap();
        println!("serialized: {bytes:?}");

        // let mut de = msg::serialization::BinaryDeserializer::new(&bytes);
        // let deserialized_param = msg::params::data::Param471::deserialize(&mut de).unwrap();
        let deserialized_param = msg::serialization::deserialize(&bytes).unwrap();
        println!("deserialized: {deserialized_param:?}");

        assert_eq!(param, deserialized_param);
    }
    #[test]
    fn test_param() {
        let param = msg::params::Param::new(
            471,
            msg::params::ParamPayload::P471(data::Param471 {
                return_code: 1,
                current_cutting_height: 2,
                default_cutting_height: 3,
                information: 1,
            }),
        );
        let bytes = param.to_bytes();
        println!("param 471 bytes: {bytes:?}");
        let p1 = msg::params::Param::from_bytes(&bytes).unwrap();
        assert_eq!(p1.id, 471);
        match p1.data {
            msg::params::ParamPayload::P471(data) => {
                assert_eq!(data.return_code, 1);
                assert_eq!(data.current_cutting_height, 2);
                assert_eq!(data.default_cutting_height, 3);
                assert_eq!(data.information, 1);
            }
            _ => panic!("unexpected param payload"),
        }
    }
    #[test]
    fn test_payload() {
        init_tracing();
        let mut payload = msg::payload::Payload::new();
        payload.msg_id = 5;
        payload.unencrypted_length = 0;
        payload.add_param(msg::params::Param::new(
            470,
            msg::params::ParamPayload::P470,
        ));
        payload.add_param(msg::params::Param::new(
            471,
            msg::params::ParamPayload::P471(data::Param471 {
                return_code: 1,
                current_cutting_height: 2,
                default_cutting_height: 3,
                information: 1,
            }),
        ));
        let bytes = payload.to_bytes();
        println!("payload bytes: {bytes:?}");
        let p1 = msg::payload::Payload::from_bytes(&bytes);
        assert_eq!(p1.msg_id, 5);
        assert_eq!(p1.params.len(), 2);
        let params = p1.get_params();
        params.iter().for_each(|param| {
            info!("param: {}, {:?}", param.id, param.data);
        });
    }
}
