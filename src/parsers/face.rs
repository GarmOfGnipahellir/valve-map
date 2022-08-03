use super::common::num_i32;
use crate::types::Face;
use combine::{between, parser::char::string, token, ParseError, Parser, RangeStream};

// (x1 y1 z1)
pub(crate) fn point<'a, I>() -> impl Parser<I, Output = [i32; 3]>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    between(
        string("( "),
        string(" )"),
        num_i32()
            .skip(token(' '))
            .and(num_i32())
            .skip(token(' '))
            .and(num_i32()),
    )
    .map(|((x, y), z)| [x, y, z])
}

pub(crate) fn triangle<'a, I>() -> impl Parser<I, Output = [[i32; 3]; 3]>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    point()
        .skip(token(' '))
        .and(point())
        .skip(token(' '))
        .and(point())
        .map(|((p1, p2), p3)| [p1, p2, p3])
}

// (x1 y1 z1) (x2 y2 z2) (x3 y3 z3) TEXTURE_NAME [ ux uy uz offsetX ] [ vx vy vz offsetY ] rotation scaleX scaleY
// fn face<'a, I>() -> impl Parser<I, Output = Face>
// where
//     I: RangeStream<Token = char, Range = &'a str>,
//     I::Error: ParseError<I::Token, I::Range, I::Position>,
// {
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_point() {
        assert_eq!(point().parse("( 128 128 16 )").unwrap().0, [128, 128, 16]);
        assert_eq!(
            point().parse("( -128 -128 -16 )").unwrap().0,
            [-128, -128, -16]
        );
    }

    #[test]
    fn test_triangle() {
        assert_eq!(
            triangle()
                .parse("( 128 128 16 ) ( 128 130 16 ) ( 130 128 16 )")
                .unwrap()
                .0,
            [[128, 128, 16], [128, 130, 16], [130, 128, 16]]
        );
        assert_eq!(
            triangle()
                .parse("( -128 -128 -16 ) ( -128 -126 -16 ) ( -128 -128 -15 )")
                .unwrap()
                .0,
            [[-128, -128, -16], [-128, -126, -16], [-128, -128, -15]]
        );
    }
}
