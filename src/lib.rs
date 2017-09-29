//! A crate with extended formatting options for ceratin types.

extern crate core;

mod slice;
mod hexdump;

pub use slice::*;
pub use hexdump::*;

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         println!("{}", hexdump(&[1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16]));
//     }
// }