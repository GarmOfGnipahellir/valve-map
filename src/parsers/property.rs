use nom::{bytes::complete::take_while, character::complete::char, IResult};

pub(crate) fn text(i: &str) -> IResult<&str, &str> {
    let (i, _) = char('"')(i)?;
    let (i, o) = take_while(|c: char| c.is_alphanumeric() || c == '_')(i)?;
    let (i, _) = char('"')(i)?;
    Ok((i, o))
}

pub(crate) fn property(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, key) = text(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, val) = text(i)?;
    Ok((i, (key, val)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text() {
        assert_eq!(text(r#""foo""#), Ok(("", "foo")));
        assert_eq!(text(r#""foo" "bar""#), Ok((r#" "bar""#, "foo")));
    }

    #[test]
    fn test_property() {
        assert_eq!(
            property(r#""key_foo" "val_bar""#),
            Ok(("", ("key_foo", "val_bar")))
        );
        assert_eq!(property(r#""foo" "bar""#), Ok(("", ("foo", "bar"))));
    }
}
