use super::{AbnfParser};
use abnf::types::{Rule, Node};

/// Exact string matcher. Used for String and Prose Node types.
///
/// Creates a parser that takes a &str as input and returns a ParserOutput if the &str matches
/// the value provided to this function, otherwise the parser returns an error.
pub (crate) fn string_parser(rule: &Rule) -> anyhow::Result<AbnfParser> {
        match rule.node() {
            Node::String(value) => Ok(string_parser_helper(rule, value)),
            Node::Prose(value) => Ok(string_parser_helper(rule, value)),
            _ => Err(anyhow::Error::msg(format!("rule {:?} not supported by the string parser!", rule)))
        }
}

fn string_parser_helper<'a>(rule: &'a Rule, value: &'a String) -> AbnfParser<'a> {
    AbnfParser::<'a>::from_rule_and_str_parser(rule, nom::bytes::complete::tag(value.as_ref()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{ParserOutput, ParserOutputValue};
    use nom::Parser;
    use rstest::rstest;

    const match_str: &str = "Hello World!";
    const string_rule: Rule = Rule::new("string test", Node::string(match_str));
    const prose_rule: Rule = Rule::new("prose test", Node::prose(match_str));
    const other_rule: Rule = Rule::new("invalid test", Node::rulename(match_str));

    #[rstest]
    #[case(&string_rule)]
    #[case(&prose_rule)]
    fn string_parser_full_match_test(#[case] rule: &Rule) {
        let mut parser = string_parser(rule).expect("String parser should return an AbnfParser!");
        let parser_output = parser.parse("Hello World!").expect("Parser should have successfully parsed the input!");
        assert_eq!(
            ("", ParserOutput { rule_name: rule.name(), value: ParserOutputValue::Value(match_str)}), parser_output
        );
    }

    #[rstest]
    #[case(&string_rule)]
    #[case(&prose_rule)]
    fn string_parser_partial_match_test(#[case] rule: &Rule) {
        let mut parser = string_parser(rule).expect("String parser should return an AbnfParser!");
        let parser_output = parser.parse("Hello World! It's a Beautiful Day!").expect("Parser should have successfully parsed the input!");
        assert_eq!(
            (" It's a Beautiful Day!", ParserOutput { rule_name: rule.name(), value: ParserOutputValue::Value(match_str)}), parser_output
        );
    }

    #[rstest]
    #[case(&string_rule)]
    #[case(&prose_rule)]
    fn string_parser_no_match_test(#[case]rule: &Rule) {
        let input = "It's a Beautiful Day!";
        let mut parser = string_parser(rule).expect("String parser should return an AbnfParser!");
        let expected = nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag));
        assert_eq!(expected, parser.parse(input).err().unwrap());
    }

    #[test]
    fn string_parser_invalid_rule_test() {
        string_parser(&other_rule).err().expect("String parser should return an error for invalid rule type!");
    }
}