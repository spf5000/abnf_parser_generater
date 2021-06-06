use super::{SmithyParser, ParserOutput, nom_output_mapper};
use abnf::types::{Rule, Node};

/// Exact string matcher. Used for String and Prose Node types.
///
/// Creates a parser that takes a &str as input and returns a ParserOutput if the &str matches
/// the value provided to this function, otherwise the parser returns an error.
pub (crate) fn string_parser(rule: &Rule) -> anyhow::Result<SmithyParser> {
        match rule.node() {
            Node::String(value) => Ok(string_parser_helper(rule, value)),
            Node::Prose(value) => Ok(string_parser_helper(rule, value)),
            _ => anyhow::Error::msg(format!("rule {:?} not supported by the string parser!", rule))
        }
}

fn string_parser_helper<'a>(rule: &'a Rule, value: &'a String) -> SmithyParser<'a> {
    nom_output_mapper(rule.name(), nom::bytes::complete::tag(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn match_literal_matches_test() {
        let match_hello_world = string_parser("Hello World!".as_ref());
        assert_eq!(
            Ok(("", ())), match_hello_world("Hello World!")
        );
    }

    #[test]
    fn match_literal_matches_with_extra_test() {
        let match_hello_world = string_parser("Hello World!".as_ref());
        assert_eq!(
            Ok((" It's a Beautiful Day!", ())), match_hello_world("Hello World! It's a Beautiful Day!")
        );
    }

    #[test]
    fn match_literal_does_not_match() {
        let match_hello_world = string_parser("Hello World!".as_ref());
        assert_eq!(
            Err("It's a Beautiful Day!"), match_hello_world("It's a Beautiful Day!")
        );
    }
}
