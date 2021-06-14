// mod alternatives;
// mod concatination;
// mod group;
// mod node_parsers;
mod nom_output_mapper;
// mod optional;
// mod repitiion;
mod rule_name;
mod string;
mod terminal_value;

// pub(crate) use terminal_value::terminal_value_match_parser;
// use nom_output_mapper::nom_output_mapper;
pub(crate) use terminal_value::terminal_value_parser;
pub(crate) use string::string_parser;
pub(crate) use rule_name::rule_name_parser;
use abnf::types::Rule;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Clone)]
pub(crate) struct AbnfParser<'a> {
    // TODO: Revisit this type
    parser: Rc<RefCell<dyn nom::Parser<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> + 'a>>
}

impl <'a> AbnfParser<'a> {
    pub(crate) fn new<T>(parser: T) -> Self
        where T: nom::Parser<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> + 'a {
        Self {
            parser: Rc::new(RefCell::new(parser))
        }
    }

    pub(crate) fn from_rule_and_str_parser<T>(rule: &'a Rule, parser: T) -> Self
    where T: nom::Parser<&'a str, &'a str, nom::error::Error<&'a str>> + 'a {
        Self::new(parser.map(move |output| ParserOutput {rule_name: rule.name(), value: ParserOutputValue::Value(String::from(output))}))
    }

    pub(crate) fn from_rule_and_char_parser<T>(rule: &'a Rule, parser: T) -> Self
    where T: nom::Parser<&'a str, char, nom::error::Error<&'a str>> + 'a {
            Self::new(parser.map(move |output| ParserOutput {rule_name: rule.name(), value: ParserOutputValue::Value(output.to_string())}))
    }
}

impl <'a> nom::Parser<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> for AbnfParser<'a> {
    fn parse(&mut self, input: &'a str) -> nom::IResult<&'a str, ParserOutput<'a>, nom::error::Error<&'a str>> {
        self.parser.borrow_mut().parse(input)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct ParserOutput<'a> {
    rule_name: &'a str,
    value: ParserOutputValue<'a>
}

impl <'a> ParserOutput<'a> {
    pub(crate) fn new(rule: &'a Rule, value: String) -> Self {
        Self {
            rule_name: rule.name(),
            value: ParserOutputValue::Value(value)
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ParserOutputValue<'a> {
    Value(String),
    Reference(Box<ParserOutput<'a>>)
}

