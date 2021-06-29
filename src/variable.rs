use crate::{
    look_alike_symbol::look_alike_symbol, number_symbol::number_symbol, symbol::Symbol,
    text_symbol::text_symbol,
};
use nom::{branch::alt, IResult};

pub fn variable(s: &str) -> IResult<&str, Symbol> {
    alt((look_alike_symbol, text_symbol, number_symbol))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_look_alike_symbol() {
        assert_eq!(look_alike_symbol("  \\   "), Ok(("", Symbol::SpaceShort)));
    }

    #[test]
    fn is_text_symbol() {
        assert_eq!(text_symbol("    AA "), Ok(("", Symbol::ForAll)));
        assert_eq!(text_symbol(" x  "), Ok(("", Symbol::Text("x"))));
    }

    #[test]
    fn is_number_symbol() {
        assert_eq!(number_symbol("42"), Ok(("", Symbol::Number("42"))));
    }
}
