use combine::{from_str, parser::range::take_while1, ParseError, Parser, RangeStream};

pub(crate) fn num_i32<'a, I>() -> impl Parser<I, Output = i32>
where
    I: RangeStream<Token = char, Range = &'a str>,
    I::Error: ParseError<I::Token, I::Range, I::Position>,
{
    from_str(take_while1(|c: char| c == '-' || c.is_digit(10)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_num_i32() {
        assert_eq!(num_i32().parse("8").unwrap().0, 8);
        assert_eq!(num_i32().parse("-8").unwrap().0, -8);
        assert_eq!(num_i32().parse("42").unwrap().0, 42);
        assert_eq!(num_i32().parse("-42").unwrap().0, -42);
    }
}
