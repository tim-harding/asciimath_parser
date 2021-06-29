use nom::{
    character::complete::multispace0, error::ParseError, sequence::delimited, AsChar, IResult,
    InputTakeAtPosition, Parser,
};

pub fn whitespaced<I, O, E, F>(f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputTakeAtPosition,
    E: ParseError<I>,
    F: Parser<I, O, E>,
    <I as InputTakeAtPosition>::Item: AsChar + Clone,
{
    delimited(multispace0, f, multispace0)
}
