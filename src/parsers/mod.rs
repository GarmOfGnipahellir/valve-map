pub(crate) mod brush;
pub(crate) mod common;
pub(crate) mod entity;
pub(crate) mod face;
pub(crate) mod map;
pub(crate) mod property;

use crate::Map;
use anyhow::{anyhow, Result};

/// # Examples
/// ```
/// use valve_map::from_str;
/// 
/// let input = include_str!("../../examples/basic.map");
/// let map = from_str(input).unwrap();
/// println!("{:#?}", map);
/// ```
pub fn from_str(s: &str) -> Result<Map> {
    map::map(s)
        .map(|(_, map)| map)
        .map_err(|err| anyhow!("{:?}", err))
}


/// # Examples
/// ```
/// use valve_map::from_bytes;
/// 
/// let input = include_bytes!("../../examples/basic.map");
/// let map = from_bytes(input).unwrap();
/// println!("{:#?}", map);
/// ```
pub fn from_bytes(b: &[u8]) -> Result<Map> {
    from_str(std::str::from_utf8(b)?)
}

/// # Examples
/// ```
/// use std::fs::File;
/// use valve_map::from_reader;
/// 
/// let mut file = File::open("examples/basic.map").unwrap();
/// let map = from_reader(&mut file).unwrap();
/// println!("{:#?}", map);
/// ```
pub fn from_reader<R>(r: &mut R) -> Result<Map>
where
    R: std::io::Read,
{
    let mut b = Vec::new();
    r.read_to_end(&mut b)?;
    from_bytes(&b)
}
