use super::AbnfParser;
use abnf::types::{Rule, Node, TerminalValues};

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

    const MATCH_STR: &str = "Hello World!";

    #[rstest]
    #[case(Rule::new("string test", Node::string(MATCH_STR)))]
    #[case(Rule::new("prose test", Node::prose(MATCH_STR)))]
    fn string_parser_full_match_test(#[case] rule: Rule) {
        let mut parser = string_parser(&rule).expect("String parser should return an AbnfParser!");
        let parser_output = parser.parse("Hello World!").expect("Parser should have successfully parsed the input!");
        assert_eq!(
            ("", ParserOutput { rule_name: rule.name(), value: ParserOutputValue::Value(MATCH_STR)}), parser_output
        );
    }

    #[rstest]
    #[case(Rule::new("string test", Node::string(MATCH_STR)))]
    #[case(Rule::new("prose test", Node::prose(MATCH_STR)))]
    fn string_parser_partial_match_test(#[case] rule: Rule) {
        let mut parser = string_parser(&rule).expect("String parser should return an AbnfParser!");
        let parser_output = parser.parse("Hello World! It's a Beautiful Day!").expect("Parser should have successfully parsed the input!");
        assert_eq!(
            (" It's a Beautiful Day!", ParserOutput { rule_name: rule.name(), value: ParserOutputValue::Value(MATCH_STR)}), parser_output
        );
    }

    #[rstest]
    #[case(Rule::new("string test", Node::string(MATCH_STR)))]
    #[case(Rule::new("prose test", Node::prose(MATCH_STR)))]
    fn string_parser_no_match_test(#[case]rule: Rule) {
        let input = "It's a Beautiful Day!";
        let mut parser = string_parser(&rule).expect("String parser should return an AbnfParser!");
        let expected = nom::Err::Error(nom::error::Error::new(input, nom::error::ErrorKind::Tag));
        assert_eq!(expected, parser.parse(input).err().unwrap());
    }

    #[test]
    fn string_parser_invalid_rule_test() {
        let test_rule = Rule::new("invalid test", Node::rulename(MATCH_STR));
        string_parser(&test_rule).err().expect("String parser should return an error for invalid rule type!");
    }
}
