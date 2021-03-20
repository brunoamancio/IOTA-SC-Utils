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