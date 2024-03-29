use wasmlib::*;

/// Panics if caller is not the contract creator
pub fn caller_must_be_contract_creator(ctx: &ScFuncContext){
    let caller_agent_id = ctx.caller();
    let contract_creator_agent_id = ctx.contract_creator();
    ctx.require(caller_agent_id == contract_creator_agent_id, "Only the contract creator may call this function!");
}

/// Panics if caller is not the chain owner
pub fn caller_must_be_chain_owner(ctx: &ScFuncContext){
    let caller_agent_id = ctx.caller();
    let chain_owner_agent_id = ctx.chain_owner_id();
    ctx.require(caller_agent_id == chain_owner_agent_id, "Only the chain owner may call this function.");
}

/// Panics if caller is not the contract itself
pub fn caller_must_be_contract_itself(ctx: &ScFuncContext){
    let caller_agent_id = ctx.caller();
    let contract_itself_agent_id = ctx.account_id();
    ctx.require(caller_agent_id == contract_itself_agent_id, "Only the contract itself may call this function!");
}