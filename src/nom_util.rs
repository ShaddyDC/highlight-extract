use nom::{
    bytes::complete::take_until, error::ParseError, FindSubstring, IResult, InputLength, InputTake,
    Parser,
};

// Note that this function may search the entire input to the end repeatedly.
// It also does some unnecessary clones.
// This seems to be fine for my purposes, but reader beware.
pub fn take_until_multiple<I, E>(matches: &[I]) -> impl FnMut(I) -> IResult<I, I, E> + '_
where
    I: Clone + InputTake + InputLength + FindSubstring<I> + HasLen,
    E: ParseError<I>,
{
    |input| {
        matches
            .iter()
            .map(|s| take_until::<I, I, E>(s.clone()).parse(input.clone()))
            .min_by_key(|v| v.as_ref().map(|(_, s)| s.len()).unwrap_or(usize::MAX))
            .expect("array should not be empty")
    }
}

pub trait HasLen {
    fn len(&self) -> usize;
}

impl HasLen for &str {
    fn len(&self) -> usize {
        str::len(self)
    }
}

#[test]
fn factor_test() {
    use nom::{
        error::{Error, ErrorKind},
        Err,
    };

    fn take(input: &str) -> IResult<&str, &str> {
        take_until_multiple(&["M1", "M2"])(input)
    }

    assert_eq!(
        take("match M1 in the middle"),
        Ok(("M1 in the middle", "match "))
    );
    assert_eq!(
        take("match M2 in the middle"),
        Ok(("M2 in the middle", "match "))
    );
    assert_eq!(
        take("no matches"),
        Err(Err::Error(Error::new("no matches", ErrorKind::TakeUntil)))
    );
}
