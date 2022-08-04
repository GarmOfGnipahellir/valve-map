use anyhow::Error;
use std::{collections::HashMap, str::FromStr};

#[derive(Debug, Default, PartialEq)]
pub struct Face {
    pub triangle: [[f32; 3]; 3],
    pub texture_name: String,
    pub axis_u: [f32; 3],
    pub axis_v: [f32; 3],
    pub offset: [f32; 2],
    pub rotation: f32,
    pub scale: [f32; 2],
}

#[derive(Debug, Default, PartialEq)]
pub struct Brush {
    pub faces: Vec<Face>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Entity {
    pub properties: HashMap<String, String>,
    pub brushes: Vec<Brush>,
}

#[derive(Debug, Default, PartialEq)]
pub struct Map {
    pub entities: Vec<Entity>,
}

impl FromStr for Map {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        crate::from_str(s)
    }
}
