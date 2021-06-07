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

// pub(crate) use terminal_value::terminal_value_match_parser;
// use nom_output_mapper::nom_output_mapper;
pub(crate) use string::string_parser;
use abnf::types::Rule;
use std::boxed::Box;

pub struct AbnfParser<'a> {
    parser: Box<dyn nom::Parser<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> + 'a>
}

impl <'a> AbnfParser<'a> {
    pub(crate) fn from_rule_and_str_parser<T>(rule: &'a Rule, parser: T) -> Self
    where T: nom::Parser<&'a str, &'a str, nom::error::Error<&'a str>> + 'a {
        Self {
            parser: Box::new(parser.map(move |output| ParserOutput {rule_name: rule.name(), value: ParserOutputValue::Value(output)}))
        }
    }
}

impl <'a> nom::Parser<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> for AbnfParser<'a> {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> {
        self.parser.parse(input)
    }
}

pub(crate) struct ParserOutput<'a> {
    rule_name: &'a str,
    value: ParserOutputValue<'a>
}

pub(crate) enum ParserOutputValue<'a> {
    Value(&'a str),
    Reference(Box<ParserOutput<'a>>)
}

