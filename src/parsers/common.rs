use nom::{
    branch::alt,
    bytes::complete::{is_a, is_not, tag, take_while},
    character::complete::char,
    combinator::eof,
    multi::many0,
    IResult,
};

pub(crate) fn num_f32(i: &str) -> IResult<&str, f32> {
    let (i, o) = take_while(|c: char| c.is_numeric() || c == '-' || c == '.' || c == 'e')(i)?;
    Ok((
        i,
        o.parse::<f32>()
            .map_err(|_| nom::Err::Error(nom::error::make_error(i, nom::error::ErrorKind::Fail)))?,
    ))
}

pub(crate) fn text(i: &str) -> IResult<&str, &str> {
    is_not(" \t\r\n")(i)
}

pub(crate) fn string(i: &str) -> IResult<&str, &str> {
    let (i, _) = char('"')(i)?;
    let (i, o) = is_not("\"")(i)?;
    let (i, _) = char('"')(i)?;
    Ok((i, o))
}

pub(crate) fn comment(i: &str) -> IResult<&str, &str> {
    let (i, _) = alt((tag("// "), tag("//")))(i)?;
    let (i, o) = alt((is_not("\r\n"), eof))(i)?;
    let (i, _) = alt((tag("\r\n"), tag("\n"), eof))(i)?;
    Ok((i, o))
}

pub(crate) fn ignored(i: &str) -> IResult<&str, Vec<&str>> {
    many0(alt((comment, is_a(" \t\r\n"))))(i)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_f32() {
        assert_eq!(num_f32("8"), Ok(("", 8.0)));
        assert_eq!(num_f32("-8"), Ok(("", -8.0)));
        assert_eq!(num_f32("42"), Ok(("", 42.0)));
        assert_eq!(num_f32("-42"), Ok(("", -42.0)));
        assert_eq!(num_f32("11.5"), Ok(("", 11.5)));
        assert_eq!(num_f32("-11.5"), Ok(("", -11.5)));
        assert_eq!(num_f32("32.125"), Ok(("", 32.125)));
        assert_eq!(num_f32("-32.125"), Ok(("", -32.125)));
        assert_eq!(
            num_f32("-1.8369701987210297e-16"),
            Ok(("", -1.8369701987210297e-16))
        );
    }

    #[test]
    fn test_text() {
        assert_eq!(text("fooBAR"), Ok(("", "fooBAR")));
        assert_eq!(text("FOO bar"), Ok((" bar", "FOO")));
        assert_eq!(text("FOO_bar"), Ok(("", "FOO_bar")));
        assert_eq!(text("FOO-bar"), Ok(("", "FOO-bar")));
        assert_eq!(text("FOO.bar"), Ok(("", "FOO.bar")));
        assert_eq!(text("FOO/bar"), Ok(("", "FOO/bar")));
    }

    #[test]
    fn test_string() {
        assert_eq!(string(r#""fooBAR""#), Ok(("", "fooBAR")));
        assert_eq!(string(r#""foo" "bar""#), Ok((r#" "bar""#, "foo")));
        assert_eq!(string(r#""foo bar""#), Ok(("", "foo bar")));
        assert_eq!(string(r#""a_B-c.D*e""#), Ok(("", "a_B-c.D*e")));
    }

    #[test]
    fn test_comment() {
        assert_eq!(comment("// foo"), Ok(("", "foo")));
        assert_eq!(comment("// foo\nbar"), Ok(("bar", "foo")));
        assert_eq!(comment("// foo\r\nbar"), Ok(("bar", "foo")));
        assert_eq!(comment("//foo&%*^bar"), Ok(("", "foo&%*^bar")));
        assert_eq!(comment("// foo: bar"), Ok(("", "foo: bar")));
    }

    #[test]
    fn test_ignored() {
        assert_eq!(ignored("// foo\n//bar"), Ok(("", vec!["foo", "bar"])));
        assert_eq!(ignored("  "), Ok(("", vec!["  "])));
        assert_eq!(
            ignored("// foo\n   \t//bar"),
            Ok(("", vec!["foo", "   \t", "bar"]))
        );
        assert_eq!(
            ignored(
                r#"// Game: Eternal Combat
// Format: Valve
// entity 0"#
            ),
            Ok((
                "",
                vec!["Game: Eternal Combat", "Format: Valve", "entity 0"]
            ))
        );
    }
}
