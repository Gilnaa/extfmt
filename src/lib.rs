//! A crate with extended formatting options for ceratin types.

extern crate core;

mod slice;
mod hexdump;

pub use slice::*;
pub use hexdump::*;