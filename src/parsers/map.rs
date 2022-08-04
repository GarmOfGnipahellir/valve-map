use crate::types::Map;
use nom::{combinator::iterator, sequence::preceded, IResult};

use super::{common::ignored, entity::entity};

pub(crate) fn map(i: &str) -> IResult<&str, Map> {
    let mut iter = iterator(i, preceded(ignored, entity));
    let entities = iter.collect();
    // FIXME: iter.finish() returns err, but we get a valid map for now atleast
    Ok(("", Map { entities }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Brush, Entity, Face};
    use std::collections::HashMap;

    #[test]
    fn test_map() {
        let i = r#"// Game: Eternal Combat
// Format: Valve
// entity 0
{
"mapversion" "220"
"classname" "worldspawn"
// brush 0
{
( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 ) __TB_empty [ 0 -0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1
( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 ) __TB_empty [ 0 -0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1
}
}
// entity 1
{
"classname" "info_player_start"
"origin" "0 0 44"
}
"#;
        let entity1 = {
            let mut properties = HashMap::new();
            properties.insert("classname".to_string(), "info_player_start".to_string());
            properties.insert("origin".to_string(), "0 0 44".to_string());
            Entity {
                properties,
                brushes: Vec::new(),
            }
        };
        let entity0 = {
            let mut properties = HashMap::new();
            properties.insert("mapversion".to_string(), "220".to_string());
            properties.insert("classname".to_string(), "worldspawn".to_string());
            Entity {
                properties,
                brushes: vec![Brush {
                    faces: vec![
                        Face {
                            triangle: [
                                [-128.0, -128.0, -16.0],
                                [-128.0, -126.0, -16.0],
                                [-128.0, -128.0, -15.0],
                            ],
                            texture_name: "__TB_empty".to_string(),
                            axis_u: [0.0, -0.5, 0.0],
                            axis_v: [0.0, 0.0, -1.0],
                            offset: [0.0, 0.0],
                            rotation: 0.0,
                            scale: [1.0, 1.0],
                        },
                        Face {
                            triangle: [
                                [-128.0, -128.0, -16.0],
                                [-128.0, -126.0, -16.0],
                                [-128.0, -128.0, -15.0],
                            ],
                            texture_name: "__TB_empty".to_string(),
                            axis_u: [0.0, -0.5, 0.0],
                            axis_v: [0.0, 0.0, -1.0],
                            offset: [0.0, 0.0],
                            rotation: 0.0,
                            scale: [1.0, 1.0],
                        },
                    ],
                }],
            }
        };
        assert_eq!(
            map(i),
            Ok((
                "",
                Map {
                    entities: vec![entity0, entity1]
                }
            ))
        )
    }
}
