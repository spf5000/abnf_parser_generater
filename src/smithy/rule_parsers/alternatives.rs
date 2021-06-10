// use super::AbnfParser;
// use abnf::types::{Rule, Node};
// 
// /// Exact string matcher. Used for String and Prose Node types.
// ///
// /// Creates a parser that takes a &str as input and returns a ParserOutput if the &str matches
// /// the value provided to this function, otherwise the parser returns an error.
// pub (crate) fn string_parser(rule: &Rule) -> anyhow::Result<AbnfParser> {
//         match rule.node() {
//             Node::String(value) => Ok(string_parser_helper(rule, value)),
//             Node::Prose(value) => Ok(string_parser_helper(rule, value)),
//             _ => Err(anyhow::Error::msg(format!("rule {:?} not supported by the string parser!", rule)))
//         }
// }
// 
// fn string_parser_helper<'a>(rule: &'a Rule, value: &'a String) -> AbnfParser<'a> {
//     AbnfParser::<'a>::from_rule_and_str_parser(rule, nom::bytes::complete::tag(value.as_ref()))
// }
// 
