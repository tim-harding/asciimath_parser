use nom::{
    branch::alt,
    bytes::complete::{is_not},
    character::complete::{char},
    combinator::{map, value},
    multi::fold_many0,
    sequence::{delimited, preceded},
    IResult,
};

use crate::{literal::Literal, misc::whitespaced};

pub fn raw_text(s: &str) -> IResult<&str, Literal> {
    whitespaced(map(parse_string, |s| Literal::RawText(s)))(s)
}

fn parse_escaped_char(s: &str) -> IResult<&str, char> {
    preceded(
        char('\\'),
        alt((value('\\', char('\\')), value('"', char('"')))),
    )(s)
}

fn parse_literal(s: &str) -> IResult<&str, &str> {
    is_not("\"\\")(s)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum StringFragment<'a> {
    Literal(&'a str),
    EscapedChar(char),
}

fn parse_fragment<'a>(s: &'a str) -> IResult<&'a str, StringFragment<'a>> {
    alt((
        map(parse_literal, StringFragment::Literal),
        map(parse_escaped_char, StringFragment::EscapedChar),
    ))(s)
}

fn parse_string(s: &str) -> IResult<&str, String> {
    delimited(char('"'), build_string, char('"'))(s)
}

fn build_string(s: &str) -> IResult<&str, String> {
    fold_many0(parse_fragment, String::new(), |mut string, fragment| {
        match fragment {
            StringFragment::Literal(s) => string.push_str(s),
            StringFragment::EscapedChar(c) => string.push(c),
        }
        string
    })(s)
}

#[cfg(test)]
mod tests {
    use crate::{literal::Literal, raw_text::raw_text};

    #[test]
    fn is_raw_text() {
        assert_eq!(
            raw_text(" \"Say \\\"things and stuff\\\"!!!\"  "),
            Ok((
                "",
                Literal::RawText("Say \"things and stuff\"!!!".to_string())
            ))
        );
    }
}
