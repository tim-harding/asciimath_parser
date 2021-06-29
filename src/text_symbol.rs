use crate::{literal::Literal, misc::whitespaced};
use nom::{
    bytes::complete::{tag, take_while, take_while1},
    combinator::map,
    sequence::delimited,
    IResult,
};

pub fn raw_text(s: &str) -> IResult<&str, Literal> {
    whitespaced(map(
        delimited(tag("\""), take_while(is_alpha), tag("\"")),
        |s| Literal::RawText(s),
    ))(s)
}

pub fn text_symbol(s: &str) -> IResult<&str, Literal> {
    whitespaced(map(take_while1(is_alpha), map_text_symbol))(s)
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn map_text_symbol(s: &str) -> Literal {
    use Literal::*;

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
        "rarr" | "rightarrow" | "to" => Right,
        "rightarrowtail" => RightTail,
        "twoheadrightarrow" => RightDouble,
        "twoheadrightarrowtail" => RightDoubleTail,
        "mapsto" => MapsTo,
        "larr" | "leftarrow" => Left,
        "harr" | "leftrightarrow" => LeftRight,
        "rArr" | "Rightarrow" | "implies" => RightBig,
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
        "if" => If,
        "iif" => IfAndOnlyIf,
        "AA" | "forall" => ForAll,
        "EE" | "exists" => Exists,
        "bot" => UpTack,
        "TT" | "top" => DownTack,
        "vdash" => Turnstile,
        "models" => Models,

        s => Symbol(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_greek_symbol() {
        assert_eq!(text_symbol("    upsilon "), Ok(("", Literal::Upsilon)));
    }

    #[test]
    fn is_text_symbol() {
        assert_eq!(text_symbol(" dy  "), Ok(("", Literal::Symbol("dy"))));
    }

    #[test]
    fn is_raw_text() {
        assert_eq!(
            raw_text(" \"things and stuff\"  "),
            Ok(("", Literal::RawText("things and stuff")))
        );
    }
}
