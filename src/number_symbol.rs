use crate::{literal::Literal, misc::whitespaced};
use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::char,
    combinator::{map, opt, recognize},
    sequence::pair,
    IResult,
};

pub fn number_symbol(s: &str) -> IResult<&str, Literal> {
    whitespaced(map(
        recognize(pair(digits, opt(pair(char('.'), digits)))),
        |s| Literal::Number(s),
    ))(s)
}

fn digits(s: &str) -> IResult<&str, &str> {
    take_while1(is_digit)(s)
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_number_symbol() {
        assert_eq!(number_symbol(" 12.4"), Ok(("", Literal::Number("12.4"))));
        assert_eq!(number_symbol(" 0.72  "), Ok(("", Literal::Number("0.72"))));
    }
}
