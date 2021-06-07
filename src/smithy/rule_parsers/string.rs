use super::{AbnfParser, nom_output_mapper};
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

    const match_str: &str = "Hello World!";
    const string_rule: Rule = Rule::new("string test", Node::String(String::from(match_str)));
    const prose_rule: Rule = Rule::new("prose test", Node::Prose(String::from(match_str)));
    const other_rule: Rule = Rule::new("invalid test", Node::Rulename(String::from(match_str)));

    #[rstest(rule, expected,
    case(string_rule, Ok((""), ()))
    case(prose_rule, Ok((""), ()))
    case(other_rule, Err("TODO"))
    )]
    fn match_literal_matches_test(rule: Rule, expected: ) {
        let match_hello_world = string_parser("Hello World!".into());
        assert_eq!(
            Ok(("", ())), match_hello_world("Hello World!")
        );
    }

    #[rstest(rule, expected,
    case(string_rule, Ok((" It's a Beautiful Day!"), ()))
    case(prose_rule, Ok((" It's a Beautiful Day!"), ()))
    case(other_rule, Err("TODO"))
    )]
    fn match_literal_matches_with_extra_test() {
        let match_hello_world = string_parser("Hello World!".into());
        assert_eq!(
            Ok((" It's a Beautiful Day!", ())), match_hello_world("Hello World! It's a Beautiful Day!")
        );
    }

    #[test]
    fn match_literal_does_not_match() {
        let match_hello_world = string_parser("Hello World!".into());
        assert_eq!(
            Err("It's a Beautiful Day!"), match_hello_world("It's a Beautiful Day!")
        );
    }
}
