use nom::{
    bytes::complete::take_while, character::complete::multispace0, combinator::map,
    sequence::delimited, IResult,
};

mod error;
mod font;
mod symbol;

use error::AmError;
use font::Font;
use symbol::Symbol;

// Goals:
// - Syntax highlighting
// - Conversion to other formats
// - Common math representation as a rendering source

// Take text while ascii alpha
// Single character, take as variable
// Multiple character, take as greek
// Failing that, parser error

fn text_var(s: &str) -> IResult<&str, Symbol> {
    map(
        delimited(multispace0, take_while(is_alpha), multispace0),
        map_text_var,
    )(s)
}

fn is_alpha(c: char) -> bool {
    c.is_ascii_alphabetic()
}

fn map_text_var(s: &str) -> Symbol {
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

        s => TextVar(s),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(Ok(("", Symbol::Upsilon)), text_var("    upsilon "));
        assert_eq!(Ok(("", Symbol::RightBig)), text_var("rArr "));
        assert_eq!(Ok(("", Symbol::TextVar("dy"))), text_var("dy"))
    }
}
