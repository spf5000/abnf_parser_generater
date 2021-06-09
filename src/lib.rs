// use abnf::rulelist;
// use std::fs;

use std::boxed::Box;

mod matchers;
mod functors;
pub mod smithy;

use functors::conditional_functor::conditional;
use functors::map_functor::map;

pub type ParseResult<'a, Output> = Result<(&'a str, Output), &'a str>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output>;
    fn map<F, NewOutput> (self, map_fn: F) -> BoxedParser<'a, NewOutput>
        where
            Self: Sized + 'a,
            Output: 'a,
            NewOutput: 'a,
            F: Fn(Output) -> NewOutput + 'a {
        BoxedParser::new(map(self, map_fn))
    }
    fn conditional<F>(self, condition_check: F) -> BoxedParser<'a, Output>
        where
            Self: Sized + 'a,
            Output: 'a,
            F: Fn(&Output) -> bool + 'a {
        BoxedParser::new(conditional(self, condition_check))
    }
}

impl <'a, F, Output> Parser<'a, Output> for F where F: Fn(&'a str) -> ParseResult<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    parser: Box<dyn Parser<'a, Output> + 'a>
}

impl <'a, Output> BoxedParser<'a, Output> {
    pub fn new<P>(parser: P) -> BoxedParser<'a, Output> where P: Parser<'a, Output> + 'a
    {
        BoxedParser {
            parser: Box::new(parser)
        }
    }
}

impl <'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParseResult<'a, Output> {
        self.parser.parse(input)
    }
}
