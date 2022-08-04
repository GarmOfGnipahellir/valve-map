//! Parser for Quake .MAP files. Only supports the Valve format (mapversion 220).
//!
//! # Basic Usage
//! ```
//! use valve_map::from_str;
//!
//! let input = r#"
//! // entity 0
//! {
//!     "mapversion" "220"
//!     "classname" "worldspawn"
//!     // brush 0
//!     {
//!         ( -16 -64 -16 ) ( -16 -63 -16 ) ( -16 -64 -15 ) __TB_empty [ 0 -1 0 -0 ] [ 0 0 -1 -0 ] -0 1 1
//!         ( -64 -16 -16 ) ( -64 -16 -15 ) ( -63 -16 -16 ) __TB_empty [ 1 0 0 -0 ] [ 0 0 -1 -0 ] -0 1 1
//!         ( -64 -64 -16 ) ( -63 -64 -16 ) ( -64 -63 -16 ) __TB_empty [ -1 0 0 0 ] [ 0 -1 0 0 ] 0 1 1
//!         ( 64 64 16 ) ( 64 65 16 ) ( 65 64 16 ) __TB_empty [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 1 1
//!         ( 64 16 16 ) ( 65 16 16 ) ( 64 16 17 ) __TB_empty [ -1 0 0 -0 ] [ 0 0 -1 -0 ] -0 1 1
//!         ( 16 64 16 ) ( 16 64 17 ) ( 16 65 16 ) __TB_empty [ 0 1 0 -0 ] [ 0 0 -1 -0 ] -0 1 1
//!     }
//! }
//! // entity 1
//! {
//!     "spawnflags" "0"
//!     "classname" "info_player_start"
//!     "origin" "32 32 24"
//! }
//! "#;
//!
//! let map = from_str(input).unwrap();
//! println!("{:#?}", map);
//! ```

mod parsers;
mod types;

pub use parsers::{from_bytes, from_reader, from_str};
pub use types::*;
