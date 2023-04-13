use nom::{
    bytes::complete::take_until, error::ParseError, FindSubstring, IResult, InputLength, Parser,
};

pub fn take_until_multiple<I, E>(matches: &[I]) -> impl FnMut(I) -> IResult<I, I, E> + '_
where
    I: Clone + nom::InputTake + InputLength + FindSubstring<I> + HasLen,
    E: ParseError<I>,
{
    |input| {
        matches
            .iter()
            .map(|s| take_until::<I, I, E>(s.clone()).parse(input.clone()))
            .min_by_key(|v| v.as_ref().map(|(_, s)| s.len()).unwrap_or(usize::MAX)) // TODO 0 is wrong
            .expect("array should not be empty")
    }
}

pub trait HasLen {
    fn len(&self) -> usize;
}

impl HasLen for &str {
    fn len(&self) -> usize {
        str::len(&self)
    }
}
