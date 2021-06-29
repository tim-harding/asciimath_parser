use nom::{
    bytes::complete::{tag, take_while, take_while1},
    character::complete::multispace0,
    combinator::{map, opt, recognize},
    error::ParseError,
    sequence::{delimited, pair},
    AsChar, IResult, InputTakeAtPosition, Parser,
};

mod error;
mod font;
mod symbol;

use symbol::Symbol;

fn numeric_constant(s: &str) -> IResult<&str, Symbol> {
    whitespaced(map(
        recognize(pair(digits, opt(pair(tag("."), digits)))),
        |s| Symbol::Number(s),
    ))(s)
}

fn digits(s: &str) -> IResult<&str, &str> {
    take_while1(is_digit)(s)
}

fn is_digit(c: char) -> bool {
    c.is_ascii_digit()
}

fn text_constant(s: &str) -> IResult<&str, Symbol> {
    map(whitespaced(take_while1(is_alpha)), map_text_constant)(s)
}

pub fn whitespaced<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    E: ParseError<I>,
    F: Parser<I, O, E>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, f, multispace0)
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn map_text_constant(s: &str) -> Symbol {
    use Symbol::*;

    match s {
        "alpha" => Alpha,
        "beta" => Beta,
        "gamma" => Gamma,
        "Gamma" => GammaBig,
        "delta" => Delta,
        "Delta" => BigDelta,
        "epsilon" => Epsilon,
        "varepsilon" => EpsilonVar,
        "zeta" => Zeta,
        "eta" => Eta,
        "theta" => Theta,
        "Theta" => ThetaBig,
        "vartheta" => ThetaVar,
        "iota" => Iota,
        "kappa" => Kappa,
        "lambda" => Lambda,
        "Lambda" => LambdaBig,
        "mu" => Mu,
        "nu" => Nu,
        "xi" => Xi,
        "Xi" => XiBig,
        "pi" => Pi,
        "Pi" => PiBig,
        "rho" => Rho,
        "sigma" => Sigma,
        "Sigma" => SigmaBig,
        "tau" => Tau,
        "upsilon" => Upsilon,
        "phi" => Phi,
        "Phi" => PhiBig,
        "varphi" => PhiVar,
        "chi" => Chi,
        "psi" => Psi,
        "Psi" => PsiBig,
        "omega" => Omega,
        "Omega" => OmegaBig,

        "uarr" | "uparrow" => Up,
        "darr" | "downarrow" => Down,
        "rarr" | "rightarrow" => Right,
        "to" => To,
        "rightarrowtail" => RightTail,
        "twoheadrightarrow" => RightDouble,
        "twoheadrightarrowtail" => RightDoubleTail,
        "mapsto" => MapsTo,
        "larr" | "leftarrow" => Left,
        "harr" | "leftrightarrow" => LeftRight,
        "rArr" | "Rightarrow" => RightBig,
        "lArr" | "Leftarrow" => LeftBig,
        "hArr" | "Leftrightarrow" => LeftRightBig,

        s => Text(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_greek_symbol() {
        assert_eq!(Ok(("", Symbol::Upsilon)), text_constant("    upsilon "));
    }

    #[test]
    fn is_text_symbol() {
        assert_eq!(Ok(("", Symbol::Text("dy"))), text_constant(" dy  "));
    }

    #[test]
    fn is_number_symbol() {
        assert_eq!(Ok(("", Symbol::Number("12.4"))), numeric_constant(" 12.4"));
        assert_eq!(
            Ok(("", Symbol::Number("0.72"))),
            numeric_constant(" 0.72  ")
        );
    }
}
