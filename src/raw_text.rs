use nom::{
    branch::alt,
    bytes::complete::{escaped_transform, is_not},
    character::complete::char,
    combinator::{map, value},
    sequence::delimited,
    IResult,
};

use crate::{literal::Literal, misc::whitespaced};

pub fn raw_text(s: &str) -> IResult<&str, Literal> {
    whitespaced(map(parse_string, |s| Literal::RawText(s)))(s)
}

fn parse_string(s: &str) -> IResult<&str, String> {
    delimited(char('"'), build_string, char('"'))(s)
}

fn build_string(s: &str) -> IResult<&str, String> {
    escaped_transform(
        is_not("\\\""),
        '\\',
        alt((value("\\", char('\\')), value("\"", char('\"')))),
    )(s)
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
