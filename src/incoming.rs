use wasmlib::*;

/// Returns the amount of tokens of 'color' attached to the call.
pub fn balance(color : &ScColor, ctx : &ScFuncContext) -> i64 {
    let balance : i64 = ctx.incoming().balance(color);
    balance
}

/// Returns a list of colors of the tokens attached to the call.
pub fn colors(ctx : &ScFuncContext) -> ScImmutableColorArray {
    let incoming_colors : ScImmutableColorArray = ctx.incoming().colors();
    incoming_colors
}

// Panics if incoming balance of 'color' is less than 'minimum_balance'.
pub fn require_balance(minimum_balance : i64, color : &ScColor, ctx : &ScFuncContext) {
    let incoming_balance = balance(color, ctx);
    ctx.require(incoming_balance >= minimum_balance, &format!("Insuficient incoming balance of color {}. Minimum required: {}.", color.to_string(), minimum_balance));
}