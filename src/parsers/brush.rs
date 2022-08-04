use crate::types::Brush;
use nom::{character::streaming::char, multi::many1, sequence::delimited, IResult};

use super::{common::ignored, face::face};

pub(crate) fn brush(i: &str) -> IResult<&str, Brush> {
    let (i, _) = char('{')(i)?;
    let (i, faces) = many1(delimited(ignored, face, ignored))(i)?;
    let (i, _) = char('}')(i)?;
    Ok((i, Brush { faces }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Face;

    #[test]
    fn test_brush() {
        let i = r#"{
( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 ) __TB_empty [ 0 -0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1
// comment
( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 ) __TB_empty [ 0 -0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1
}"#;
        assert_eq!(
            brush(i),
            Ok((
                "",
                Brush {
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
                }
            ))
        )
    }
}
