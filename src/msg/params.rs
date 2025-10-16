mod param470;
#[repr(u16)]
enum ParamId {
    GetCuttingHeight{req: (), res: param470::Param471} = 470,
}
pub struct Param {
    id: ParamId,
    len: u16,
}