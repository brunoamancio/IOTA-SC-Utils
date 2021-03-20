[![Actions Status](https://github.com/brunoamancio/IOTA-SC-Utils/workflows/Build/badge.svg)](https://github.com/brunoamancio/IOTA-SC-Utils/actions)

# IOTA-SC-Utils

This toolkit provides simple interfaces to IOTA smart contract developers, as well as a modified version of [wasmlib](https://github.com/iotaledger/wasp/tree/master/contracts/rust/wasmlib/docs) with minor changes with added syntax sugar only.

## Add to Cargo.toml (ensure you add the latest version)
```
[dependencies]
iota_sc_utils = { git = "https://github.com/brunoamancio/IOTA-SC-Utils", tag = "v0.8.8"}
```

## A few samples:

### Access: Verify access rights.
```
fn my_iota_sc_function(ctx : &ScFuncContext){
    /// Panics if caller is not the contract creator
    access::caller_must_be_contract_creator(ctx);

    /// Panics if caller is not the chain owner
    access::caller_must_be_chain_owner(ctx);

    /// Panics if caller is not the contract itself
    access::caller_must_be_contract_itself(ctx);
}
```

---

### Params: Parameters passed when calling SC function.
```
fn my_iota_sc_function(ctx : &ScFuncContext){
    // Tries to get a parameter. Panics if it can't find it.
    let my_input_param1 : String = params::must_get_string("param_name1", ctx);

    // Tries to get a parameter. Returns default value if it can't find it.
    let my_input_param2 : String = params::get_string("param_name2", ctx);

    // Checks if a parameter exists. Returns true if it exists.
    let param_exists : bool = params::exists_string("param_name3", ctx);
}
```
**Supports:** String, int64, bytes, ScAgentId, ScAddress, ScRequestId, ScHname, ScHash, ScContractId, ScColor, and ScChainId.

---

### State: Variables saved in SC state.
```
fn my_iota_sc_function(ctx : &ScFuncContext){
    // Tries to get a variable. Panics if it can't find it.
    let my_variable1 : String = state::must_get_string("variable_name1", ctx);

    // Tries to get a variable. Returns default value if it can't find it.
    let my_variable2 : String = state::get_string("variable_name2", ctx);

    // Checks if a variable exists. Returns true if it exists.
    let variable_exists : bool = state::exists_string("variable_name3", ctx);
}
```
**Supports:** String, int64, bytes, ScAgentId, ScAddress, ScRequestId, ScHname, ScHash, ScContractId, ScColor, and ScChainId.

---

### Results: 
- Values returned to sc function caller after the request is processed.
```
fn my_iota_sc_function(ctx : &ScFuncContext){
    // Sets value in response structure
    results::set_string("my_returned_string_var_name", "my string value", ctx);

    // Sets value in response structure
    results::set_int64("my_returned_i64_var_name", 10_i64, ctx);
}
```
- Values returned from calls to other contracts.
```
fn my_iota_sc_function(ctx : &ScFuncContext){
    let call_result : ScImmutableMap = ctx.call(<hname_contract>, <hname_func>, None);
    
    /// Tries to get a returned value. Panics if it can't find it.
    let my_string1 = results::must_get_string("my_returned_string_var_name1", call_result);

    /// Tries to get a parameter. Returns default value if it can't find it.
    let my_string2 = results::get_string("my_returned_string_var_name2", call_result);

    /// Checks if a parameter exists. Returns true if it exists.
    let returned_variable_exists : bool = results::exists_string("my_returned_string_var_name3", call_result);
}
```
**Supports:** String, int64, bytes, bool, ScAgendId, ScAddress, ScRequestId, ScHname, ScHash, ScContractId, ScColor, and ScChainId.

---

### Incoming: Tokens attached to the call

```
fn my_iota_sc_function(ctx : &ScFuncContext){
    /// Returns the amount of tokens of 'color' attached to the call. In this case, incoming amount of IOTA tokens.
    let incoming_balance : i64 = incoming::balance(&ScColor::IOTA, ctx);

    /// Returns a list of colors of the tokens attached to the call.
    let incoming_colors : ScImmutableColorArray = incoming::colors(ctx);
}
```

---

### SafeMath: Safe math operations which panic on under/overflows (stops SC execution)
- Addition:
```
let a := 1_i64;
// Syntax 1
let added_value1 = a.safe_add(2, ctx);
// Syntax 2
let added_value2 = math::safe_add(a, 2, ctx);
```
- Subtraction:
```
let a = 1_i64;
// Syntax 1
let sub_value1 = a.safe_sub(2, ctx);
// Syntax 2
let sub_value2 = math::safe_sub(a, 2, ctx);
```
- Multiplication:
```
let a = 1_i64;
// Syntax 1
let mul_value1 = a.safe_mul(2, ctx);
// Syntax 2
let mul_value2 = math::safe_mul(a, 2, ctx);
```
- Division:
```
let a = 1_i64;
// Syntax 1
let div_value1 = a.safe_div(2, ctx);
// Syntax 2
let div_value2 = math::safe_div(a, 2, ctx);
```
---
**Supports:** i8, i16, i32, i64, isize, u8, u16, u32, u64, usize;

### Hname generator
Generates Schnames and/or u32 representation thereof in compile-time. 
Sample:
```
use iota_sc_utils::generator;
// Generated in compile-time!
const HNAME_MY_FUNC : ScHname = generator::generate_schname!("my_func");
```
- Github repository: https://github.com/brunoamancio/IOTA-SC-HName-Generator 
- Documentation: https://crates.io/crates/iota-sc-hname-generator

---

### Wasmlib
[Modified](https://github.com/brunoamancio/Wasm-lib) version of IOTA Foundation's [wasmlib](https://github.com/iotaledger/wasp/tree/master/contracts/rust/wasmlib). There are no logic changes, only additions to support syntax sugar. The reason is that, unfortunately, IOTA Foundation takes long to evaluate pull requests to their repository, even for small additions. Current feature:
- Support for seemless xor operation between ScHnames with (Pending PR on the [official repo](https://github.com/iotaledger/wasp/pull/132)):
```
let hname1 : ScHname(0x0000001);
let hname2 : ScHname(0x0000002);
let xor : ScHname = hname1 ^ hname2;
```

Usage sample of the integrated wasmlib:
```
use iota_sc_utils::wasmlib::{ScExports, ScViewContext};

#[no_mangle]
pub fn on_load() {
    let exports = ScExports::new();
    exports.add_view("hello_view", hello_view);
}

pub fn hello_view(ctx : &ScViewContext) {
    // Hello!
}
```
