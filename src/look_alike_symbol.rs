use nom::{branch::alt, bytes::complete::tag, combinator::value, IResult};

use crate::{misc::whitespaced, symbol::Symbol};

pub fn look_alike_symbol(s: &str) -> IResult<&str, Symbol> {
    whitespaced(alt((
        look_alike_symbol_1,
        look_alike_symbol_2,
        look_alike_symbol_3,
    )))(s)
}

fn look_alike_symbol_1(s: &str) -> IResult<&str, Symbol> {
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

fn look_alike_symbol_2(s: &str) -> IResult<&str, Symbol> {
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

fn look_alike_symbol_3(s: &str) -> IResult<&str, Symbol> {
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

fn if_and_only_if(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::IfAndOnlyIf, tag("<=>"))(s)
}

fn bowtie(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Bowtie, tag("|><|"))(s)
}

fn right_double_tail(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::RightDoubleTail, tag(">->>"))(s)
}

fn right_tail(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::RightTail, tag(">->"))(s)
}

fn right_double(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::RightDouble, tag("->>"))(s)
}

fn maps_to(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::MapsTo, tag("|->"))(s)
}

fn star(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Star, tag("***"))(s)
}

fn times_left(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::TimesLeft, tag("|><"))(s)
}

fn times_right(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::TimesRight, tag("><|"))(s)
}

fn wedge_big(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::WedgeBig, tag("^^^"))(s)
}

fn dots_low(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::DotsLow, tag("..."))(s)
}

fn triangle(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Triangle, tag("/_\\"))(s)
}

fn floor_left(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::FloorLeft, tag("|__"))(s)
}

fn floor_right(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::FloorLeft, tag("__|"))(s)
}

fn preceded_equal(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::PrecededEqual, tag("-<="))(s)
}

fn succeeded(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Succeeded, tag(">-"))(s)
}

fn succeeded_equal(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::SucceededEqual, tag(">-="))(s)
}

fn in_not(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::InNot, tag("!in"))(s)
}

fn up_tack(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::UpTack, tag("_|_"))(s)
}

fn turnstile(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Turnstile, tag("|--"))(s)
}

fn models(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Models, tag("|=="))(s)
}

fn asterisk(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Asterisk, tag("**"))(s)
}

fn slash(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Slash, tag("//"))(s)
}

fn backslash(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Slash, tag("\\\\"))(s)
}

fn divide(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Divide, tag("-:"))(s)
}

fn circle_plus(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::CirclePlus, tag("o+"))(s)
}

fn circle_dot(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::CircleDot, tag("o."))(s)
}

fn wedge(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Wedge, tag("^^"))(s)
}

fn plus_minus(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::PlusMinus, tag("+-"))(s)
}

fn set_empty(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::SetEmpty, tag("O/"))(s)
}

fn therefore(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Therefore, tag(":."))(s)
}

fn because(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Because, tag(":'"))(s)
}

fn space_short(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::SpaceShort, tag("\\ "))(s)
}

fn angle(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Angle, tag("/_"))(s)
}
fn ceiling_left(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::CeilingLeft, tag("|~"))(s)
}

fn ceiling_right(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::CeilingRight, tag("~|"))(s)
}

fn equal_not(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::EqualNot, tag("!="))(s)
}

fn less_equal(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::LessEqual, tag("<="))(s)
}

fn greater_equal(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::GreaterEqual, tag(">="))(s)
}

fn preceded(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Preceded, tag("-<"))(s)
}

fn equivalent(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Equivalent, tag("-="))(s)
}

fn congruous(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Congruous, tag("~="))(s)
}

fn approximate(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Approximate, tag("~~"))(s)
}

fn implies(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::RightBig, tag("=>"))(s)
}

fn to(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Right, tag("->"))(s)
}

fn less(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Less, tag("<"))(s)
}

fn greater(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Greater, tag(">"))(s)
}

fn equal(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Equal, tag("="))(s)
}

fn circle(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Circle, tag("@"))(s)
}

fn plus(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Plus, tag("+"))(s)
}

fn minus(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::Minus, tag("-"))(s)
}

fn dot_center(s: &str) -> IResult<&str, Symbol> {
    value(Symbol::DotCenter, tag("*"))(s)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_look_alikes() {
        assert_eq!(look_alike_symbol(" ="), Ok(("", Symbol::Equal)));
        assert_eq!(look_alike_symbol("=> "), Ok(("", Symbol::RightBig)));
        assert_eq!(look_alike_symbol(">->>"), Ok(("", Symbol::RightDoubleTail)));
    }
}
