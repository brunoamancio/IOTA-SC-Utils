use wasmlib::*;
use crate::getter::Getter;
use crate::getter::PARAMS;

/// Creates a new ScMutableMap instance
pub fn new() -> ScMutableMap {
    let params = ScMutableMap::new();
    params
}

// ---------------------------    Getter functions    -------------------------------------

// Primitive types
add_impl_pub_fns!(PARAMS, must_get_string, get_string, exists_string, String);
add_impl_pub_fns!(PARAMS, must_get_int64, get_int64, exists_int64, i64);
add_impl_pub_fns!(PARAMS, must_get_bytes, get_bytes, exists_bytes, Vec<u8>);

// ISCP Types
add_impl_pub_fns!(PARAMS, must_get_agent_id, get_agent_id, exists_agent_id, ScAgentID);
add_impl_pub_fns!(PARAMS, must_get_address, get_address, exists_address, ScAddress);
add_impl_pub_fns!(PARAMS, must_get_request_id, get_request_id, exists_request_id, ScRequestID);
add_impl_pub_fns!(PARAMS, must_get_hname, get_hname, exists_hname, ScHname);
add_impl_pub_fns!(PARAMS, must_get_hash, get_hash, exists_hash, ScHash);
add_impl_pub_fns!(PARAMS, must_get_color, get_color, exists_color, ScColor);
add_impl_pub_fns!(PARAMS, must_get_chain_id, get_chain_id, exists_chain_id, ScChainID);


// ---------------------------    Add functions    -------------------------------------

macro_rules! add_impl_adder_pub_fns {

    ($add_func_name:ident, $get_func : ident, $param_type : ty) => {
        /// Sets a variable in MutableMap.
        pub fn $add_func_name(key :&str, value : $param_type, mutable_map : &ScMutableMap) {
            let hname_param = mutable_map.$get_func(key);
            hname_param.set_value(value);
        }
    };
}

// Primitive types
add_impl_adder_pub_fns!(add_string, get_string, &str);
add_impl_adder_pub_fns!(add_int64, get_int64, i64);
add_impl_adder_pub_fns!(add_bytes, get_bytes, &[u8]);

// ISCP Types
add_impl_adder_pub_fns!(add_agent_id, get_agent_id, &ScAgentID);
add_impl_adder_pub_fns!(add_address, get_address, &ScAddress);
add_impl_adder_pub_fns!(add_request_id, get_request_id, &ScRequestID); 
add_impl_adder_pub_fns!(add_hname, get_hname, ScHname);
add_impl_adder_pub_fns!(add_hash, get_hash, &ScHash);
add_impl_adder_pub_fns!(add_color, get_color, &ScColor);
add_impl_adder_pub_fns!(add_chain_id, get_chain_id, &ScChainID);
