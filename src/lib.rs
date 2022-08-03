mod des;
mod error;
mod types;
mod parsers;

use combine::{
    between,
    parser::{
        char::string,
        range::{take_while, take_while1},
    },
    token, ParseError, Parser, RangeStream,
};

fn is_space(c: char) -> bool {
    c == ' ' || c == '\t'
}

fn chars<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    take_while(|c: char| c.is_alphabetic() || c.is_alphanumeric() || is_space(c))
}

fn chars1<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    take_while1(|c: char| c.is_alphabetic() || c.is_alphanumeric() || is_space(c))
}

fn comment<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    string("//")
        .skip(take_while(|c: char| c == '/' || is_space(c)))
        .and(chars())
        .map(|(_, v)| v)
}

fn ignored<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    comment().or(take_while(|c: char| c.is_whitespace()))
}

fn text<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    between(token('"'), token('"'), chars())
}

fn text1<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    between(token('"'), token('"'), chars1())
}

fn property<'a, I>() -> impl Parser<I, Output = (&'a str, &'a str)>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    text1().skip(token(' ')).and(text())
}

fn scope<'a, I>() -> impl Parser<I, Output = &'a str>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    let mut i = 0;
    between(
        token('{'),
        token('}'),
        take_while(move |c: char| {
            let res = c != '}' || i != 0;
            if c == '{' {
                i += 1;
            } else if i > 0 && c == '}' {
                i -= 1;
            }
            res
        }),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use combine::EasyParser;

    #[test]
    fn test_chars() {
        assert_eq!(chars().easy_parse("value").unwrap().0, "value");
        assert_eq!(chars().easy_parse("foo bar").unwrap().0, "foo bar");

        assert!(chars().easy_parse("").is_ok());
        assert!(chars1().easy_parse("").is_err());
    }

    #[test]
    fn test_comment() {
        assert_eq!(comment().easy_parse("// foo\n").unwrap().0, "foo");
        assert_eq!(comment().easy_parse("// foo").unwrap().0, "foo");
        assert_eq!(comment().easy_parse("/// foo").unwrap().0, "foo");
    }

    #[test]
    fn test_ignored() {
        assert!(ignored().easy_parse("   ").is_ok());
        assert!(ignored().easy_parse("// foo").is_ok());
        assert!(ignored().easy_parse("  \n    \n   \t   \n").is_ok());
    }

    #[test]
    fn test_text() {
        assert_eq!(text().easy_parse(r#""value""#).unwrap().0, "value");
        assert_eq!(text().easy_parse(r#""foo bar""#).unwrap().0, "foo bar");

        assert!(text().easy_parse(r#""""#).is_ok());
        assert!(text1().easy_parse(r#""""#).is_err());
    }

    #[test]
    fn test_property() {
        assert_eq!(
            property().easy_parse(r#""key" "value""#).unwrap().0,
            ("key", "value")
        );
    }

    #[test]
    fn test_scope() {
        assert_eq!(scope().easy_parse("{foo}").unwrap().0, "foo");
        assert_eq!(scope().easy_parse("{foo{bar}}").unwrap().0, "foo{bar}");
        assert_eq!(
            scope().easy_parse("{foo{bar{foo}}{baz}}").unwrap().0,
            "foo{bar{foo}}{baz}"
        );
    }
}
