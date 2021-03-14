use crate::{params, results, generator, wasmlib::{ScHname, ScFuncContext}};

pub const NAME_FUNC_IMPLEMENTS : &str  = "implements";
const HNAME_FUNC_IMPLEMENTS : ScHname  =  generator::generate_schname!("implements");

pub const INTERFACE_TIP_100 : &str = "tip_100";
pub const HNAME_INTERFACE_TIP_100 : ScHname = generator::generate_schname!("implements(ScHname,ScHname)->bool");

pub fn implements(hname_contract : ScHname, hname_interface : ScHname, ctx : &ScFuncContext) -> bool {
    let input_params = params::new();
    params::add_hname(INTERFACE_TIP_100, hname_interface, &input_params);

    let result = ctx.call(hname_contract, HNAME_FUNC_IMPLEMENTS, Some(input_params), None);
    let implements = results::get_bool(NAME_FUNC_IMPLEMENTS, result);
    implements
}
