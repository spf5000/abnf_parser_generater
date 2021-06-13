use super::AbnfParser;
use abnf::types::{TerminalValues, Rule, Node};

/// Terminal Value matcher. Used for Terminal Value ranges (ex: U+0041 ('A') to U+005A ('Z')) or
/// Terminal Value vectors.
///
/// Creates a parser that takes a &str as input and returns a ParserOutput if the &str matches
/// one of the characters within the terminal value provided, otherwise the parser returns an error.
pub(crate) fn terminal_value_parser<'a>(rule: &'a Rule) -> anyhow::Result<AbnfParser<'a>> {
    if let Node::TerminalValues(terminal_value) = rule.node() {
        terminal_value_parser_helper(rule, terminal_value)
    } else {
        Err(anyhow::Error::msg(format!("rule {} not supported by terminal value parser!", rule)))
    }
}

fn terminal_value_parser_helper<'a>(rule: &'a Rule, terminal_values: &'a TerminalValues) -> anyhow::Result<AbnfParser<'a>> {
    let characters: String = match terminal_values {
        TerminalValues::Concatenation(characters) => characters.iter().map(|char_value| char::from_u32(*char_value)).map(Option::unwrap).collect(),
        TerminalValues::Range(start, end) => (*start..=*end).map(char::from_u32).map(Option::unwrap).collect(), // TODO: find better way to unwrap Option<char>;
    };

    // Need to move ownership of the terminal value String
    let nom_parser = move |input: &'a str| {
        nom::character::complete::one_of::<&'a str, &str, nom::error::Error<&'a str>>(characters.as_ref())(input)
    };

    Ok(AbnfParser::<'a>::from_rule_and_char_parser(rule, nom_parser))
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::{ParserOutput, ParserOutputValue};
    use nom::Parser;
    use nom::error::{Error as nom_Error, ErrorKind};
    use rstest::rstest;

    const test_rule_name: &str = "test_rule_name";

    #[rstest]
    #[case("Hello World!", 'A', 'Z', Ok(("ello World!", 'H')))]
    #[case("Hello World!", '1', '9', Err(nom_Error::new("Hello World!", ErrorKind::OneOf)))]
    #[case("Hello World!", 'a', 'z', Err(nom_Error::new("Hello World!", ErrorKind::OneOf)))]
    #[case("12345", '1', '9', Ok(("2345", '1')))]
    #[case("12345", 'A', 'Z', Err(nom_Error::new("12345", ErrorKind::OneOf)))]
    #[case("12345", '2', '9', Err(nom_Error::new("12345", ErrorKind::OneOf)))]
    fn terminal_value_range_matches_tests(#[case] input: &str, #[case] start: char, #[case] end: char, #[case] expected: Result<(&str, char), nom::error::Error<&str>>) {
        let terminal_value = TerminalValues::Range(u32::from(start), u32::from(end));
        terminal_value_test(input, terminal_value, expected)
    }

    #[rstest]
    #[case("abc", vec!['a', 'e', 'i', 'o', 'u'], Ok(("bc", 'a')))]
    #[case("efg", vec!['a', 'e', 'i', 'o', 'u'], Ok(("fg", 'e')))]
    #[case("ijk", vec!['a', 'e', 'i', 'o', 'u'], Ok(("jk", 'i')))]
    #[case("opq", vec!['a', 'e', 'i', 'o', 'u'], Ok(("pq", 'o')))]
    #[case("uvw", vec!['a', 'e', 'i', 'o', 'u'], Ok(("vw", 'u')))]
    #[case("xyz", vec!['a', 'e', 'i', 'o', 'u'], Err(nom_Error::new("xyz", ErrorKind::OneOf)))]
    fn terminal_value_concat_matches_tests(#[case] input: &str, #[case] chars: Vec<char>, #[case] expected: Result<(&str, char), nom::error::Error<&str>>) {
        let terminal_value = TerminalValues::Concatenation(chars.into_iter().map(u32::from).collect());
        terminal_value_test(input, terminal_value, expected)
    }

    fn terminal_value_test(input: &str, terminal_value: TerminalValues, expected: Result<(&str, char), nom::error::Error<&str>>) {
        let rule = Rule::new(test_rule_name, Node::terminal_values(terminal_value));
        let mut parser = terminal_value_parser(&rule).unwrap();
        let expected_result = expected.map(|(remaining, match_char)| {
            (remaining, ParserOutput{rule_name: test_rule_name, value: ParserOutputValue::Value(match_char.to_string())})
        }).map_err(|err| nom::Err::Error(err));
        assert_eq!(expected_result, parser.parse(input));
    }

    #[test]
    fn terminal_value_parser_invalid_rule_test() {
        let test_rule = Rule::new("invalid test", Node::rulename("INVALID!"));
        terminal_value_parser(&test_rule).err().expect("Terminal Value parser should return an error for invalid rule type!");
    }
}

