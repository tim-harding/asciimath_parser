use crate::error::AmError;
use std::convert::TryFrom;

pub enum Font {
    Bold,
    Blackboard,
    Caligraphic,
    Typewriter,
    Fraktur,
    Sans,
}

impl TryFrom<&str> for Font {
    type Error = AmError;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        use Font::*;

        match s {
            "bb" => Ok(Bold),
            "bbb" => Ok(Blackboard),
            "cc" => Ok(Caligraphic),
            "tt" => Ok(Typewriter),
            "fr" => Ok(Fraktur),
            "sf" => Ok(Sans),
            _ => Err(AmError::Font),
        }
    }
}

impl Into<&str> for Font {
    fn into(self) -> &'static str {
        match self {
            Font::Bold => "bb",
            Font::Blackboard => "bbb",
            Font::Caligraphic => "cc",
            Font::Typewriter => "tt",
            Font::Fraktur => "fr",
            Font::Sans => "sf",
        }
    }
}
