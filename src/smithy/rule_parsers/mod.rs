mod alternatives;
mod concatination;
mod group;
mod node_parsers;
mod nom_output_mapper;
mod optional;
mod repitiion;
mod rule_name;
mod string;
mod terminal_value;

pub(crate) use terminal_value::terminal_value_match_parser;
pub(crate) use string::string_parser;
use nom_output_mapper::nom_output_mapper;
use std::boxed::Box;

pub(crate) type AbnfParser<'a> = nom::Parser<&'a str, ParserOutput<'a>, (&'a str, nom::error::ErrorKind)>;
// pub(crate) type SmithyParser<'a> = impl nom::Parser<&'a str, ParserOutput<'a>, anyhow::Error>;
// pub(crate) type SmithyParseResult<'a> = crate::ParseResult<'a, ParserOutput<'a>>;
// pub(crate) type SmithyParser<'a> = crate::Parser<'a, ParserOutput<'a>>;
// pub(crate) type SmithyParseResult<'a, O> = Result<(&'a str, O), anyhow::Error>;
// pub(crate) type SmithyParser<'a, O> = Box<dyn Fn(&str) -> SmithyParseResult<'a, O>>;

pub(crate) struct ParserOutput<'a> {
    rule_name: &'a str,
    value: ParserOutputValue<'a>
}

pub(crate) enum ParserOutputValue<'a> {
    Value(&'a str),
    Reference(Box<ParserOutput<'a>>)
}

