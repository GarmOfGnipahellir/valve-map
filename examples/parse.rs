use combine::{
    parser::range::{range, take_while1},
    sep_by, EasyParser, between, token,
};

fn main() {
    let input = r#""value""#;
    let mut string = between(token('"'), token('"'), take_while1(|c: char| c.is_alphabetic()));
    let output = string.easy_parse(input).unwrap().0;
    println!("{output}");
}
