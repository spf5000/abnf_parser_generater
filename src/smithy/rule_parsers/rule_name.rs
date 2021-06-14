use super::{AbnfParser, ParserOutput, ParserOutputValue};
use std::collections::HashMap;
use abnf::types::{Rule, Node};
use nom::Parser;

/// Rule Name parser. Takes a rule and a HashMap of rule names to parsers and returns a parser
/// that will call the correct sub parser for this rule's rule_name reference and return
/// a ParserOutput with a reference ParserOutputValue.
///
/// Creates a parser that takes a &str as input and returns a ParserOutput referencing the
/// output of the parser for the given rule_name of this parser if that parser succeeds. Else it
/// returns and error.
pub (crate) fn rule_name_parser<'a>(rule: &'a Rule, rule_parser_map: &'a HashMap<&'a str, AbnfParser<'a>>) -> anyhow::Result<AbnfParser<'a>> {
        match rule.node() {
            Node::Rulename(rule_name) => {
                let sub_parser = rule_parser_map.get(rule_name.as_str())
                    .ok_or(anyhow::Error::msg(format!("Referenced rule name {} not found in rule_name_map!", rule_name)))?;
                Ok(rule_name_parser_helper(rule, sub_parser.clone()))
            },
            _ => Err(anyhow::Error::msg(format!("rule {:?} not supported by the string parser!", rule)))
        }
}

// TODO: Should have some better error handling to show the full "Rule Stack" on reference errors.
fn rule_name_parser_helper<'a>(rule: &'a Rule, sub_parser: AbnfParser<'a>) -> AbnfParser<'a> {
    let parent_rule_name = rule.name().clone();
    let mapped_parser = sub_parser.map(move |parser_output| {
        ParserOutput {
            rule_name: parent_rule_name,
            value: ParserOutputValue::Reference(Box::new(parser_output))
        }
    });
    AbnfParser::new(mapped_parser)
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{ParserOutput, ParserOutputValue, terminal_value_parser};
    use nom::Parser;
    use nom::error::{Error as nom_Error, ErrorKind};
    use rstest::rstest;
    use abnf::types::TerminalValues;

    const SUB_RULE_NAME: &str = "SUB_RULE_NAME";
    const PARENT_RULE_NAME: &str = "PARENT_RULE_NAME";

    #[rstest]
    #[case("Hello World!", 'A', 'Z', Ok(("ello World!", 'H')))]
    #[case("Hello World!", '1', '9', Err(nom_Error::new("Hello World!", ErrorKind::OneOf)))]
    #[case("Hello World!", 'a', 'z', Err(nom_Error::new("Hello World!", ErrorKind::OneOf)))]
    #[case("12345", '1', '9', Ok(("2345", '1')))]
    #[case("12345", 'A', 'Z', Err(nom_Error::new("12345", ErrorKind::OneOf)))]
    #[case("12345", '2', '9', Err(nom_Error::new("12345", ErrorKind::OneOf)))]
    fn terminal_value_range_sub_parser_tests(#[case] input: &str, #[case] start: char, #[case] end: char, #[case] expected: Result<(&str, char), nom::error::Error<&str>>) {
        let terminal_value = TerminalValues::Range(u32::from(start), u32::from(end));
        terminal_value_sub_parser_test(input, terminal_value, expected)
    }

    #[rstest]
    #[case("abc", vec!['a', 'e', 'i', 'o', 'u'], Ok(("bc", 'a')))]
    #[case("efg", vec!['a', 'e', 'i', 'o', 'u'], Ok(("fg", 'e')))]
    #[case("ijk", vec!['a', 'e', 'i', 'o', 'u'], Ok(("jk", 'i')))]
    #[case("opq", vec!['a', 'e', 'i', 'o', 'u'], Ok(("pq", 'o')))]
    #[case("uvw", vec!['a', 'e', 'i', 'o', 'u'], Ok(("vw", 'u')))]
    #[case("xyz", vec!['a', 'e', 'i', 'o', 'u'], Err(nom_Error::new("xyz", ErrorKind::OneOf)))]
    fn terminal_value_concat_sub_parser_tess(#[case] input: &str, #[case] chars: Vec<char>, #[case] expected: Result<(&str, char), nom::error::Error<&str>>) {
        let terminal_value = TerminalValues::Concatenation(chars.into_iter().map(u32::from).collect());
        terminal_value_sub_parser_test(input, terminal_value, expected)
    }

    fn terminal_value_sub_parser_test(input: &str, terminal_value: TerminalValues, expected: Result<(&str, char), nom::error::Error<&str>>) {
        let mut rule_name_map = HashMap::new();
        let sub_rule = Rule::new(SUB_RULE_NAME, Node::terminal_values(terminal_value));
        let sub_parser = terminal_value_parser(&sub_rule).expect("Terminal Value SubParser should be Ok!");
        rule_name_map.insert(SUB_RULE_NAME, sub_parser);

        // Create a rule name parser
        let rule = Rule::new(PARENT_RULE_NAME, Node::rulename(SUB_RULE_NAME));
        let mut parser = rule_name_parser(&rule, &rule_name_map).expect("Rule Name Parser should be Ok!");

        let expected_result = expected.map(|(remaining, match_char)| {
            let sub_output_value = ParserOutput{
                rule_name: SUB_RULE_NAME,
                value: ParserOutputValue::Value(match_char.to_string())
            };
            let output_value = ParserOutput{
                rule_name: PARENT_RULE_NAME, 
                value: ParserOutputValue::Reference(Box::new(sub_output_value))
            };

            (remaining, output_value)
        }).map_err(|err| nom::Err::Error(err));

        assert_eq!(expected_result, parser.parse(input));
    }

    #[test]
    fn rule_name_parser_invalid_rule_test() {
        let test_rule = Rule::new("invalid test", Node::string("INVALID!"));
        let rule_name_map = HashMap::new();
        rule_name_parser(&test_rule, &rule_name_map).err().expect("Rule Name parser should return an error for invalid rule type!");
    }
}

