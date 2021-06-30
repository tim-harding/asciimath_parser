use nom::{
    branch::alt, bytes::complete::tag, character::complete::char, combinator::value, IResult,
};

use crate::{literal::Literal, misc::whitespaced};

pub fn look_alike_symbol(s: &str) -> IResult<&str, Literal> {
    whitespaced(alt((
        look_alike_symbol_1,
        look_alike_symbol_2,
        look_alike_symbol_3,
    )))(s)
}

fn look_alike_symbol_1(s: &str) -> IResult<&str, Literal> {
    alt((
        if_and_only_if,
        bowtie,
        right_double_tail,
        right_tail,
        right_double,
        maps_to,
        star,
        times_left,
        times_right,
        wedge_big,
        dots_low,
        triangle,
        floor_left,
        floor_right,
        preceded_equal,
        succeeded,
        succeeded_equal,
        in_not,
        up_tack,
        turnstile,
        models,
    ))(s)
}

fn look_alike_symbol_2(s: &str) -> IResult<&str, Literal> {
    alt((
        asterisk,
        slash,
        backslash,
        divide,
        circle_plus,
        circle_dot,
        wedge,
        plus_minus,
        set_empty,
        therefore,
        because,
        space_short,
        angle,
        ceiling_left,
        ceiling_right,
        equal_not,
        less_equal,
        greater_equal,
        preceded,
        equivalent,
        congruous,
    ))(s)
}

fn look_alike_symbol_3(s: &str) -> IResult<&str, Literal> {
    alt((
        approximate,
        implies,
        to,
        less,
        greater,
        equal,
        circle,
        plus,
        minus,
        dot_center,
    ))(s)
}

fn if_and_only_if(s: &str) -> IResult<&str, Literal> {
    value(Literal::IfAndOnlyIf, tag("<=>"))(s)
}

fn bowtie(s: &str) -> IResult<&str, Literal> {
    value(Literal::Bowtie, tag("|><|"))(s)
}

fn right_double_tail(s: &str) -> IResult<&str, Literal> {
    value(Literal::RightDoubleTail, tag(">->>"))(s)
}

fn right_tail(s: &str) -> IResult<&str, Literal> {
    value(Literal::RightTail, tag(">->"))(s)
}

fn right_double(s: &str) -> IResult<&str, Literal> {
    value(Literal::RightDouble, tag("->>"))(s)
}

fn maps_to(s: &str) -> IResult<&str, Literal> {
    value(Literal::MapsTo, tag("|->"))(s)
}

fn star(s: &str) -> IResult<&str, Literal> {
    value(Literal::Star, tag("***"))(s)
}

fn times_left(s: &str) -> IResult<&str, Literal> {
    value(Literal::TimesLeft, tag("|><"))(s)
}

fn times_right(s: &str) -> IResult<&str, Literal> {
    value(Literal::TimesRight, tag("><|"))(s)
}

fn wedge_big(s: &str) -> IResult<&str, Literal> {
    value(Literal::WedgeBig, tag("^^^"))(s)
}

fn dots_low(s: &str) -> IResult<&str, Literal> {
    value(Literal::DotsLow, tag("..."))(s)
}

fn triangle(s: &str) -> IResult<&str, Literal> {
    value(Literal::Triangle, tag("/_\\"))(s)
}

fn floor_left(s: &str) -> IResult<&str, Literal> {
    value(Literal::FloorLeft, tag("|__"))(s)
}

fn floor_right(s: &str) -> IResult<&str, Literal> {
    value(Literal::FloorLeft, tag("__|"))(s)
}

fn preceded_equal(s: &str) -> IResult<&str, Literal> {
    value(Literal::PrecededEqual, tag("-<="))(s)
}

fn succeeded(s: &str) -> IResult<&str, Literal> {
    value(Literal::Succeeded, tag(">-"))(s)
}

fn succeeded_equal(s: &str) -> IResult<&str, Literal> {
    value(Literal::SucceededEqual, tag(">-="))(s)
}

fn in_not(s: &str) -> IResult<&str, Literal> {
    value(Literal::InNot, tag("!in"))(s)
}

fn up_tack(s: &str) -> IResult<&str, Literal> {
    value(Literal::UpTack, tag("_|_"))(s)
}

fn turnstile(s: &str) -> IResult<&str, Literal> {
    value(Literal::Turnstile, tag("|--"))(s)
}

fn models(s: &str) -> IResult<&str, Literal> {
    value(Literal::Models, tag("|=="))(s)
}

fn asterisk(s: &str) -> IResult<&str, Literal> {
    value(Literal::Asterisk, tag("**"))(s)
}

fn slash(s: &str) -> IResult<&str, Literal> {
    value(Literal::Slash, tag("//"))(s)
}

fn backslash(s: &str) -> IResult<&str, Literal> {
    value(Literal::Slash, tag("\\\\"))(s)
}

fn divide(s: &str) -> IResult<&str, Literal> {
    value(Literal::Divide, tag("-:"))(s)
}

fn circle_plus(s: &str) -> IResult<&str, Literal> {
    value(Literal::CirclePlus, tag("o+"))(s)
}

fn circle_dot(s: &str) -> IResult<&str, Literal> {
    value(Literal::CircleDot, tag("o."))(s)
}

fn wedge(s: &str) -> IResult<&str, Literal> {
    value(Literal::Wedge, tag("^^"))(s)
}

fn plus_minus(s: &str) -> IResult<&str, Literal> {
    value(Literal::PlusMinus, tag("+-"))(s)
}

fn set_empty(s: &str) -> IResult<&str, Literal> {
    value(Literal::SetEmpty, tag("O/"))(s)
}

fn therefore(s: &str) -> IResult<&str, Literal> {
    value(Literal::Therefore, tag(":."))(s)
}

fn because(s: &str) -> IResult<&str, Literal> {
    value(Literal::Because, tag(":'"))(s)
}

fn space_short(s: &str) -> IResult<&str, Literal> {
    value(Literal::SpaceShort, tag("\\ "))(s)
}

fn angle(s: &str) -> IResult<&str, Literal> {
    value(Literal::Angle, tag("/_"))(s)
}
fn ceiling_left(s: &str) -> IResult<&str, Literal> {
    value(Literal::CeilingLeft, tag("|~"))(s)
}

fn ceiling_right(s: &str) -> IResult<&str, Literal> {
    value(Literal::CeilingRight, tag("~|"))(s)
}

fn equal_not(s: &str) -> IResult<&str, Literal> {
    value(Literal::EqualNot, tag("!="))(s)
}

fn less_equal(s: &str) -> IResult<&str, Literal> {
    value(Literal::LessEqual, tag("<="))(s)
}

fn greater_equal(s: &str) -> IResult<&str, Literal> {
    value(Literal::GreaterEqual, tag(">="))(s)
}

fn preceded(s: &str) -> IResult<&str, Literal> {
    value(Literal::Preceded, tag("-<"))(s)
}

fn equivalent(s: &str) -> IResult<&str, Literal> {
    value(Literal::Equivalent, tag("-="))(s)
}

fn congruous(s: &str) -> IResult<&str, Literal> {
    value(Literal::Congruous, tag("~="))(s)
}

fn approximate(s: &str) -> IResult<&str, Literal> {
    value(Literal::Approximate, tag("~~"))(s)
}

fn implies(s: &str) -> IResult<&str, Literal> {
    value(Literal::RightBig, tag("=>"))(s)
}

fn to(s: &str) -> IResult<&str, Literal> {
    value(Literal::Right, tag("->"))(s)
}

fn less(s: &str) -> IResult<&str, Literal> {
    value(Literal::Less, char('<'))(s)
}

fn greater(s: &str) -> IResult<&str, Literal> {
    value(Literal::Greater, char('>'))(s)
}

fn equal(s: &str) -> IResult<&str, Literal> {
    value(Literal::Equal, char('='))(s)
}

fn circle(s: &str) -> IResult<&str, Literal> {
    value(Literal::Circle, char('@'))(s)
}

fn plus(s: &str) -> IResult<&str, Literal> {
    value(Literal::Plus, char('+'))(s)
}

fn minus(s: &str) -> IResult<&str, Literal> {
    value(Literal::Minus, char('-'))(s)
}

fn dot_center(s: &str) -> IResult<&str, Literal> {
    value(Literal::DotCenter, char('*'))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_look_alikes() {
        assert_eq!(look_alike_symbol(" ="), Ok(("", Literal::Equal)));
        assert_eq!(look_alike_symbol("=> "), Ok(("", Literal::RightBig)));
        assert_eq!(
            look_alike_symbol(">->>"),
            Ok(("", Literal::RightDoubleTail))
        );
    }
}
