pub(crate) mod brush;
pub(crate) mod common;
pub(crate) mod entity;
pub(crate) mod face;
pub(crate) mod map;
pub(crate) mod property;

use anyhow::{anyhow, Result};

use crate::types::Map;

pub fn map_from_str(input: &str) -> Result<Map> {
    map::map(input)
        .map(|(_, map)| map)
        .map_err(|err| anyhow!("{:?}", err))
}
