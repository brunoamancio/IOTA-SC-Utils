use crate::{params, results, generator, wasmlib::{ScHname, ScViewContext}};

/// Name of the function responsible for checking if contracts implement interfaces, as defined by the TIP-100 standard.
pub const NAME_FUNC_IMPLEMENTS : &str  = "implements";
// Hash of the name of the function "implements" described in TIP-100.
const HNAME_FUNC_IMPLEMENTS : ScHname  =  generator::generate_schname!("implements");

/// Name of the TIP-100 interface.
pub const INTERFACE_TIP_100 : &str = "tip_100";
/// Hash of TIP-100 Interface.
pub const HNAME_INTERFACE_TIP_100 : ScHname = generator::generate_schname!("implements(ScHname,ScHname)->bool");

/// Function which calls the contract with hname `hname_contract` and checks if it implements interface `hname_interface`.
/// # Sample:
/// ```ignore
/// fn my_sc_view(ctx : &ScViewContext) {
///     let hname_external_contract : ScHname = ScHname(\<contract hash\>);
///     let hname_interface_knows_how_to_bark : ScHname = ScHname(\<interface hash\>);
///     let external_contract_knows_how_to_bark : bool = interfaces::implements(hname_external_contract, hname_interface_knows_how_to_bark);
///     if external_contract_knows_how_to_bark {
///         // woof
///     }
/// }
/// ```
pub fn implements(hname_contract : ScHname, hname_interface : ScHname, ctx : &ScViewContext) -> bool {
    let input_params = params::new();
    params::add_hname(INTERFACE_TIP_100, hname_interface, &input_params);

    let result = ctx.call(hname_contract, HNAME_FUNC_IMPLEMENTS, Some(input_params));
    let implements = results::get_bool(NAME_FUNC_IMPLEMENTS, result);
    implements
}
