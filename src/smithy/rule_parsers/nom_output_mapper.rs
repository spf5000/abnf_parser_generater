use super::ParserOutput;
use crate::smithy::rule_parsers::{ParserOutputValue, SmithyParser, AbnfParser};

// pub (crate) fn nom_output_mapper<'a>(rule_name: &'a str, parser: impl nom::Parser<&'a str, (&'a str, &'a str), anyhow::Error>) -> SmithyParser<'a> {
//     nom::combinator::map(parser, |(remaining, value)| {
//         (remaining, ParserOutput { rule_name, value: ParserOutputValue::Value(value.as_ref()) })
//     })
// }
//
pub(crate) fn nom_output_mapper<'a>(rule_name: &'a str, parser: impl nom::Parser<&'a str, &'a str, (&'a str, nom::error::ErrorKind)>) -> impl AbnfParser<'a> {
    parser.map(|output| ParserOutput {rule_name, value: ParserOutputValue::Value(output)})
}
