// mod des;
// mod error;
mod parsers;
mod types;

pub use parsers::{from_bytes, from_reader, from_str};
pub use types::*;
