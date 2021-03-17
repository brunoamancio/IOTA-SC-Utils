#[macro_use]
///  Responsible for registering getter functions to contexts.
pub mod getter;
///  Responsible for handling access requirements. Fails calls on unauthorized access.
pub mod access;
///  Responsible for ensuring no under/overflows happen on maths calculations. Ensures calls panic on over/underflows.
pub mod math;

///  Syntax sugar to work with context parameters.
pub mod params;
///  Syntax sugar to work with context results.
pub mod results;
///  Syntax sugar to work with context state.
pub mod state;

///  Keeps information relevant to standards such as TIP-100.
pub mod interfaces;

/// Responsible for code generation.
/// Integrated tool responsible for generating hashes in compile-time. 
/// Documentation: https://crates.io/crates/iota-sc-hname-generator
/// Github repository: https://github.com/brunoamancio/IOTA-SC-HName-Generator
pub mod generator;

/// Integrated tool responsible for communication with ISCP's vm. This is a modified version of the original IF's wasmlib.
/// Differences are kept minimal and only exist if they provide simpler interface, nothing else. 
/// Documentation: https://github.com/iotaledger/wasp/tree/master/contracts/rust/wasmlib/docs
/// Github repository: https://github.com/brunoamancio/IOTA-SC-HName-Generator
pub mod wasmlib;