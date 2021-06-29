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

fn number_symbol(s: &str) -> IResult<&str, Symbol> {
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

fn text_symbol(s: &str) -> IResult<&str, Symbol> {
    map(whitespaced(take_while1(is_alpha)), map_text_symbol)(s)
}

fn whitespaced<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
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

fn map_text_symbol(s: &str) -> Symbol {
    use Symbol::*;

    match s {
        // Greek
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

        // Arrows
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

        // Operation
        "cdot" => DotCenter,
        "ast" => Asterisk,
        "star" => Star,
        "backslash" | "setminus" => Backslash,
        "div" => Divide,
        "times" | "xx" => Times,
        "ltimes" => TimesLeft,
        "rtimes" => TimesRight,
        "bowtie" => Bowtie,
        "circ" => Circle,
        "oplus" => CirclePlus,
        "otimes" | "ox" => CircleTimes,
        "odot" => CircleDot,
        "sum" => Summation,
        "prod" => Product,
        "wedge" => Wedge,
        "bigwedge" => WedgeBig,
        "vv" | "vee" => Vee,
        "vvv" | "bigvee" => VeeBig,
        "nn" | "cap" => Cap,
        "nnn" | "bigcap" => CapBig,
        "uu" | "cup" => Cup,
        "uuu" | "bigcup" => CupBig,
        
        // Miscellaneous
        "int" => Integral,
        "oint" => IntegralSurface,
        "del" | "partial" => Del,
        "grad" | "nabla" => Nabla,
        "pm" => PlusMinus,
        "oo" | "infty" => Infinity,
        "aleph" => Aleph,
        "therefore" => Therefore,
        "because" => Because,
        "ldots" => DotsLow,
        "cdots" => DotsCenter,
        "vdots" => DotsVertical,
        "ddots" => DotsDiagonal,
        "quad" => SpaceLong,
        "angle" => Angle,
        "frown" => Frown,
        "triangle" => Triangle,
        "diamond" => Diamond,
        "square" => Square,
        "lfloor" => FloorLeft,
        "rfloor" => FloorRight,
        "lceiling" => CeilingLeft,
        "rceiling" => CeilingRight,
        "emptyset" => SetEmpty,
        "CC" => SetComplex,
        "NN" => SetNatural,
        "QQ" => SetRational,
        "RR" => SetReal,
        "ZZ" => SetInteger,
        
        // Relations
        "ne" => EqualNot,
        "lt" => Less,
        "le" => LessEqual,
        "ll" | "mlt" => LessLess,
        "gt" => Greater,
        "ge" => GreaterEqual,
        "gg" | "mgt" => GreaterGreater,
        "prec" => Preceded,
        "preceq" => PrecededEqual,
        "succ" => Succeeded,
        "succeq" => SucceededEqual,
        "in" => In,
        "notin" => InNot,
        "sub" | "subset" => Subset,
        "sube" | "subseteq" => SubsetEqual,
        "sup" | "supset" => Superset,
        "supe" | "supseteq" => SupersetEqual,
        "equiv" => Equivalent,
        "cong" => Congruous,
        "approx" => Approximate,
        "prop" | "propto" => Proportional,
        
        // Logial
        "and" => And,
        "or" => Or,
        "not" | "neg" => Not,
        "implies" => Implies,
        "if" => If,
        "iif" => IfAndOnlyIf,
        "AA" | "forall" => ForAll,
        "EE" | "exists" => Exists,
        "bot" => UpTack,
        "TT" | "top" => DownTack,
        "vdash" => Turnstile,
        "models" => Models,

        s => Text(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_greek_symbol() {
        assert_eq!(Ok(("", Symbol::Upsilon)), text_symbol("    upsilon "));
    }

    #[test]
    fn is_text_symbol() {
        assert_eq!(Ok(("", Symbol::Text("dy"))), text_symbol(" dy  "));
    }

    #[test]
    fn is_number_symbol() {
        assert_eq!(Ok(("", Symbol::Number("12.4"))), number_symbol(" 12.4"));
        assert_eq!(
            Ok(("", Symbol::Number("0.72"))),
            number_symbol(" 0.72  ")
        );
    }
}
