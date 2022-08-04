use super::{brush::brush, common::ignored, property::property};
use crate::types::Entity;
use nom::{
    character::streaming::char, combinator::opt, multi::many1, sequence::delimited, IResult,
};
use std::collections::HashMap;

pub(crate) fn entity(i: &str) -> IResult<&str, Entity> {
    let (i, _) = char('{')(i)?;
    let (i, prop_vec) = opt(many1(delimited(ignored, property, ignored)))(i)?;
    let (i, brushes) = opt(many1(delimited(ignored, brush, ignored)))(i)?;
    let (i, _) = char('}')(i)?;

    let properties = {
        let mut properties = HashMap::new();
        if let Some(prop_vec) = prop_vec {
            for (k, v) in prop_vec {
                properties.insert(k.to_string(), v.to_string());
            }
        }
        properties
    };
    let brushes = match brushes {
        Some(brushes) => brushes,
        None => Vec::new(),
    };

    Ok((
        i,
        Entity {
            properties,
            brushes,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Brush, Face};
    use std::collections::HashMap;

    #[test]
    fn test_entity() {
        let i = r#"{
"classname" "info_player_start"
"origin" "0 0 44"
}"#;
        let mut properties = HashMap::new();
        properties.insert("classname".to_string(), "info_player_start".to_string());
        properties.insert("origin".to_string(), "0 0 44".to_string());
        assert_eq!(
            entity(i),
            Ok((
                "",
                Entity {
                    properties,
                    brushes: Vec::new(),
                }
            ))
        );

        let i = r#"{
"mapversion" "220"
"classname" "worldspawn"
// brush 0
{
( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 ) __TB_empty [ 0 -0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1
( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 ) __TB_empty [ 0 -0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1
}
}"#;
        let mut properties = HashMap::new();
        properties.insert("mapversion".to_string(), "220".to_string());
        properties.insert("classname".to_string(), "worldspawn".to_string());
        assert_eq!(
            entity(i),
            Ok((
                "",
                Entity {
                    properties,
                    brushes: vec![Brush {
                        faces: vec![
                            Face {
                                triangle: [
                                    [-128.0, -128.0, -16.0],
                                    [-128.0, -126.0, -16.0],
                                    [-128.0, -128.0, -15.0]
                                ],
                                texture_name: "__TB_empty".to_string(),
                                axis_u: [0.0, -0.5, 0.0],
                                axis_v: [0.0, 0.0, -1.0],
                                offset: [0.0, 0.0],
                                rotation: 0.0,
                                scale: [1.0, 1.0]
                            },
                            Face {
                                triangle: [
                                    [-128.0, -128.0, -16.0],
                                    [-128.0, -126.0, -16.0],
                                    [-128.0, -128.0, -15.0]
                                ],
                                texture_name: "__TB_empty".to_string(),
                                axis_u: [0.0, -0.5, 0.0],
                                axis_v: [0.0, 0.0, -1.0],
                                offset: [0.0, 0.0],
                                rotation: 0.0,
                                scale: [1.0, 1.0]
                            }
                        ]
                    }],
                }
            ))
        );
    }
}
