use wasmlib::*;

/// Describes a source of data to be loaded. In this case, from a context's state.
pub const STATE : &str = "state";
/// Describes a source of data to be loaded. In this case, from a context's params.
pub const PARAMS : &str = "params";

macro_rules! add_impl_pub_fns {

    ($source:tt, $must_get_func_name:ident, $get_func_name:ident, $exists_func_name:ident, $return_type:ty) => {
        /// Tries to get a variable. Panics if it can't find it.
        pub fn $must_get_func_name<TGetter:Getter>(variable_name : &str, ctx : &TGetter) -> $return_type {
            ctx.$must_get_func_name($source, variable_name)
        }

        /// Tries to get a variable. Returns default value if it can't find it.
        pub fn $get_func_name<TGetter:Getter>(variable_name : &str, ctx : &TGetter) -> $return_type {
            ctx.$get_func_name($source, variable_name)
        }

        /// Checks if a variable exists. Returns true if it exists.
        pub fn $exists_func_name<TGetter:Getter>(variable_name : &str, ctx : &TGetter) -> bool {
            ctx.$exists_func_name($source, variable_name)
        }
    };
}

macro_rules! add_all_getter_fns {
    ($must_get_func_name:ident, $get_func_name:ident, $exists_func_name:ident, $return_type:ty) => {
        /// Tries to get a variable. Panics if it can't find it.
        fn $must_get_func_name(&self, source : &str, variable_name : &str) -> $return_type;
        /// Tries to get a variable. Returns default value if it can't find it.
        fn $get_func_name(&self, source : &str, variable_name : &str) -> $return_type;

        fn $exists_func_name(&self, source : &str, variable_name : &str) -> bool;
    };

    () => {
        // Primitive types
        add_all_getter_fns!(must_get_string, get_string, exists_string, String);
        add_all_getter_fns!(must_get_int64, get_int64, exists_int64, i64);
        add_all_getter_fns!(must_get_bytes, get_bytes, exists_bytes, Vec<u8>);

        // ISCP types
        add_all_getter_fns!(must_get_agent_id, get_agent_id, exists_agent_id,  ScAgentID);
        add_all_getter_fns!(must_get_address, get_address, exists_address, ScAddress);
        add_all_getter_fns!(must_get_request_id, get_request_id, exists_request_id, ScRequestID);
        add_all_getter_fns!(must_get_hname, get_hname, exists_hname, ScHname);
        add_all_getter_fns!(must_get_hash, get_hash, exists_hash, ScHash);
        add_all_getter_fns!(must_get_color, get_color, exists_color, ScColor);
        add_all_getter_fns!(must_get_chain_id, get_chain_id, exists_chain_id, ScChainID);
    };
}

/// Defines get and must_get operations for primitive as well as for ISCP properties.
pub trait Getter {
    add_all_getter_fns!();
}

macro_rules! add_impl_getters {
    ($must_get_func_name:ident, $get_func_name:ident, $exists_func_name:ident, $return_type:ty) => {
        
        /// Tries to get a variable. Panics if it can't find it.
        fn $must_get_func_name(&self, source : &str, variable_name : &str) -> $return_type {
            match source {
                crate::getter::STATE =>  {
                    let variable = self.state().$get_func_name(variable_name);
                    crate::getter::require_if_needed(self, variable.exists(), &format!("variable {} not found", variable_name));
                    variable.value()
                },
                crate::getter::PARAMS => {
                    let param = self.params().$get_func_name(variable_name);
                    crate::getter::require_if_needed(self, param.exists(), &format!("parameter {} not found", variable_name));
                    param.value()
                },
                _ => panic!("Source {} not implemented", source),
            }
        }

        /// Tries to get a variable. Returns default value if it can't find it.
        fn $get_func_name(&self, source : &str, variable_name : &str) -> $return_type {
            match source {
                crate::getter::STATE =>  {
                    self.state().$get_func_name(variable_name).value()
                },
                crate::getter::PARAMS => {
                    self.params().$get_func_name(variable_name).value()
                },
                _ => panic!("Source {} not implemented", source),
            }
        }

        fn $exists_func_name(&self, source : &str, variable_name : &str) -> bool {
            match source {
                crate::getter::STATE =>  {
                    self.state().$get_func_name(variable_name).exists()
                },
                crate::getter::PARAMS => {
                    self.params().$get_func_name(variable_name).exists()
                },
                _ => panic!("Source {} not implemented", source),
            }
        }
    };

    ($context:ty) => {
        impl Getter for $context {
            // Primitive types
            add_impl_getters!(must_get_string, get_string, exists_string, String);
            add_impl_getters!(must_get_int64, get_int64, exists_int64, i64);
            add_impl_getters!(must_get_bytes, get_bytes, exists_bytes, Vec<u8>);
        
            // ISCP types
            add_impl_getters!(must_get_agent_id, get_agent_id, exists_agent_id, ScAgentID);
            add_impl_getters!(must_get_address, get_address, exists_address, ScAddress);
            add_impl_getters!(must_get_request_id, get_request_id, exists_request_id, ScRequestID);
            add_impl_getters!(must_get_hname, get_hname, exists_hname, ScHname);
            add_impl_getters!(must_get_hash, get_hash, exists_hash, ScHash);
            add_impl_getters!(must_get_color, get_color, exists_color, ScColor);
            add_impl_getters!(must_get_chain_id, get_chain_id, exists_chain_id, ScChainID);
        }
    };
}   

add_impl_getters!(ScFuncContext);
add_impl_getters!(ScViewContext);

/// Require the condition is true for the context. Panic if false.
pub fn require_if_needed<TContext:ScBaseContext>(context : &TContext, condition : bool, error_message : &str) {
    context.require(condition, error_message);
}
