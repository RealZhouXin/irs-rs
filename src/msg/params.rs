pub mod data;
#[repr(u16)]
enum ParamId {
    GetCuttingHeight{req: (), res: data::Param471} = 470,
}
pub struct Param {
    id: ParamId,
    len: u16,
}