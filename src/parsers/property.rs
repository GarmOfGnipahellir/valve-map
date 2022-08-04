use super::common::string;
use nom::{character::complete::char, IResult};

pub(crate) fn property(i: &str) -> IResult<&str, (&str, &str)> {
    let (i, key) = string(i)?;
    let (i, _) = char(' ')(i)?;
    let (i, val) = string(i)?;
    Ok((i, (key, val)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_property() {
        assert_eq!(
            property(r#""key_foo" "val_bar""#),
            Ok(("", ("key_foo", "val_bar")))
        );
        assert_eq!(property(r#""foo" "bar""#), Ok(("", ("foo", "bar"))));
    }
}
