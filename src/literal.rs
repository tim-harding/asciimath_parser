use crate::{
    look_alike_symbol::look_alike_symbol, number_symbol::number_symbol, raw_text::raw_text,
    text_symbol::text_symbol,
};
use nom::{branch::alt, IResult};

pub fn literal(s: &str) -> IResult<&str, Literal> {
    alt((look_alike_symbol, raw_text, text_symbol, number_symbol))(s)
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Literal<'a> {
    Symbol(&'a str),
    Number(&'a str),
    RawText(String),

    // Greek
    Alpha,
    Beta,
    Gamma,
    GammaBig,
    Delta,
    BigDelta,
    Epsilon,
    EpsilonVar,
    Zeta,
    Eta,
    Theta,
    ThetaBig,
    ThetaVar,
    Iota,
    Kappa,
    Lambda,
    LambdaBig,
    Mu,
    Nu,
    Xi,
    XiBig,
    Pi,
    PiBig,
    Rho,
    Sigma,
    SigmaBig,
    Tau,
    Upsilon,
    Phi,
    PhiBig,
    PhiVar,
    Chi,
    Psi,
    PsiBig,
    Omega,
    OmegaBig,

    // Arrow
    Up,
    Down,
    Left,
    LeftBig,
    Right,
    RightBig,
    RightTail,
    RightDouble,
    RightDoubleTail,
    LeftRight,
    LeftRightBig,
    MapsTo,

    // Operation
    Plus,
    Minus,
    DotCenter,
    Asterisk,
    Star,
    Slash,
    Backslash,
    Divide,
    Times,
    TimesLeft,
    TimesRight,
    Bowtie,
    Circle,
    CirclePlus,
    CircleTimes,
    CircleDot,
    Summation,
    Product,
    Wedge,
    WedgeBig,
    Vee,
    VeeBig,
    Cap,
    CapBig,
    Cup,
    CupBig,

    // Miscellaneous
    Integral,
    IntegralSurface,
    Del,
    Nabla,
    PlusMinus,
    Infinity,
    Aleph,
    Therefore,
    Because,
    DotsLow,
    DotsCenter,
    DotsVertical,
    DotsDiagonal,
    SpaceShort,
    SpaceLong,
    Angle,
    Frown,
    Triangle,
    Diamond,
    Square,
    FloorLeft,
    FloorRight,
    CeilingLeft,
    CeilingRight,
    SetEmpty,
    SetComplex,
    SetNatural,
    SetRational,
    SetReal,
    SetInteger,

    // Relations
    Equal,
    EqualNot,
    Less,
    LessEqual,
    LessLess,
    Greater,
    GreaterEqual,
    GreaterGreater,
    Preceded,
    PrecededEqual,
    Succeeded,
    SucceededEqual,
    In,
    InNot,
    Subset,
    SubsetEqual,
    Superset,
    SupersetEqual,
    Equivalent,
    Congruous,
    Approximate,
    Proportional,

    // Logical
    And,
    Or,
    Not,
    If,
    IfAndOnlyIf,
    ForAll,
    Exists,
    UpTack,
    DownTack,
    Turnstile,
    Models,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_look_alike_symbol() {
        assert_eq!(literal("  \\   "), Ok(("", Literal::SpaceShort)));
    }

    #[test]
    fn is_text_symbol() {
        assert_eq!(literal("    AA "), Ok(("", Literal::ForAll)));
        assert_eq!(literal(" x  "), Ok(("", Literal::Symbol("x"))));
    }

    #[test]
    fn is_number_symbol() {
        assert_eq!(literal("42"), Ok(("", Literal::Number("42"))));
    }

    #[test]
    fn is_raw_text() {
        assert_eq!(
            literal("\"Hello\""),
            Ok(("", Literal::RawText("Hello".to_string())))
        );
    }
}
