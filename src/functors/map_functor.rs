use crate::{Parser};

pub fn map<'a, P, M, A, B>(parser: P, mapper: M) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    M: Fn(A) -> B,
{
    move |input| {
        parser.parse(input).map(|(remaining, original)| (remaining, mapper(original)))
    }
}