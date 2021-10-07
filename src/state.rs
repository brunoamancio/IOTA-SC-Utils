use wasmlib::*;
use crate::getter::Getter;
use crate::getter::STATE;

// Primitive types
add_impl_pub_fns!(STATE, must_get_string, get_string, exists_string, String);
add_impl_pub_fns!(STATE, must_get_int64, get_int64, exists_int64, i64);
add_impl_pub_fns!(STATE, must_get_bytes, get_bytes, exists_bytes, Vec<u8>);

// ISCP Types
add_impl_pub_fns!(STATE, must_get_agent_id, get_agent_id, exists_agent_id, ScAgentID);
add_impl_pub_fns!(STATE, must_get_address, get_address, exists_address, ScAddress);
add_impl_pub_fns!(STATE, must_get_request_id, get_request_id, exists_request_id, ScRequestID);
add_impl_pub_fns!(STATE, must_get_hname, get_hname, exists_hname, ScHname);
add_impl_pub_fns!(STATE, must_get_hash, get_hash, exists_hash, ScHash);
add_impl_pub_fns!(STATE, must_get_color, get_color, exists_color, ScColor);
add_impl_pub_fns!(STATE, must_get_chain_id, get_chain_id, exists_chain_id, ScChainID);
