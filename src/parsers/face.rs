use super::common::{num_f32, text};
use crate::types::Face;
use nom::{bytes::complete::tag, character::streaming::char, IResult};

pub(crate) fn vec3(i: &str) -> IResult<&str, [f32; 3]> {
    let (i, x) = num_f32(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, y) = num_f32(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, z) = num_f32(i)?;
    Ok((i, [x, y, z]))
}

pub(crate) fn triangle(i: &str) -> IResult<&str, [[f32; 3]; 3]> {
    let (i, _) = tag("( ")(i)?;
    let (i, p1) = vec3(i)?;
    let (i, _) = tag(" ) ( ")(i)?;
    let (i, p2) = vec3(i)?;
    let (i, _) = tag(" ) ( ")(i)?;
    let (i, p3) = vec3(i)?;
    let (i, _) = tag(" )")(i)?;
    Ok((i, [p1, p2, p3]))
}

// (x1 y1 z1) (x2 y2 z2) (x3 y3 z3) TEXTURE_NAME [ ux uy uz offsetX ] [ vx vy vz offsetY ] rotation scaleX scaleY
pub(crate) fn face(i: &str) -> IResult<&str, Face> {
    let (i, triangle) = triangle(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, texture_name) = text(i)?;
    let (i, _) = tag(" [ ")(i)?;
    let (i, axis_u) = vec3(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, offset_x) = num_f32(i)?;
    let (i, _) = tag(" ] [ ")(i)?;
    let (i, axis_v) = vec3(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, offset_y) = num_f32(i)?;
    let (i, _) = tag(" ] ")(i)?;
    let (i, rotation) = num_f32(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, scale_x) = num_f32(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, scale_y) = num_f32(i)?;
    Ok((
        i,
        Face {
            triangle,
            texture_name: texture_name.to_string(),
            axis_u,
            axis_v,
            offset: [offset_x, offset_y],
            rotation,
            scale: [scale_x, scale_y],
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
        assert_eq!(vec3("-128 -128 -15.5"), Ok(("", [-128.0, -128.0, -15.5])));
        assert_eq!(vec3("130 128 16"), Ok(("", [130.0, 128.0, 16.0])));
    }

    #[test]
    fn test_triangle() {
        assert_eq!(
            triangle("( -128 -128 -15.5 ) ( -126 -128 -15.5 ) ( -128 -126 -15.5 )"),
            Ok((
                "",
                [
                    [-128.0, -128.0, -15.5],
                    [-126.0, -128.0, -15.5],
                    [-128.0, -126.0, -15.5]
                ]
            ))
        );
    }

    #[test]
    fn test_face() {
        assert_eq!(
            face("( 128 128 16 ) ( 128 128 17 ) ( 128 130 16 ) __TB_empty [ 0 0.5 0 0 ] [ 0 0 -1 0 ] 0 1 1"), 
            Ok((
                "", 
                Face {
                    triangle: [
                        [128.0, 128.0, 16.0],
                        [128.0, 128.0, 17.0],
                        [128.0, 130.0, 16.0],
                    ],
                    texture_name: "__TB_empty".to_string(),
                    axis_u: [0.0, 0.5, 0.0],
                    axis_v: [0.0, 0.0, -1.0],
                    offset: [0.0, 0.0],
                    rotation: 0.0,
                    scale: [1.0, 1.0]
                }
            ))
        )
    }
}
